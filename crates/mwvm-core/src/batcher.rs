//! Continuous Batching Service — production-grade local inference batcher.
//!
//! Dynamic batch formation with timeout + size threshold (vLLM/TGI-style).
//! Requests are submitted via [`ContinuousBatcher::infer`] and processed by a
//! background tokio task in configurable micro-batches.

use std::sync::Arc;
use std::time::Duration;

use flume::{Receiver, Sender};
use tokio::sync::oneshot;
use tokio::time::Instant;
use tracing::{debug, error};

use morpheum_primitives::vm::types::InferenceRequest;

use crate::{MwvmError, Result};

// ─── Public API ────────────────────────────────────────────────────────────

/// Production continuous batcher for local inference.
#[derive(Clone)]
pub struct ContinuousBatcher {
    tx: Sender<BatcherRequest>,
    _worker: Arc<tokio::task::JoinHandle<()>>,
}

/// Internal request envelope sent to the background worker.
struct BatcherRequest {
    req: InferenceRequest,
    responder: oneshot::Sender<Result<Vec<u8>>>,
}

impl ContinuousBatcher {
    /// Spawn a new batcher with a background worker task.
    #[must_use]
    pub fn new() -> Self {
        let (tx, rx) = flume::bounded(1024);
        let worker = tokio::spawn(worker_loop(rx));
        Self {
            tx,
            _worker: Arc::new(worker),
        }
    }

    /// Submit an inference request and await the result.
    ///
    /// # Errors
    ///
    /// Returns an error if the batcher channel is closed or the response is dropped.
    pub async fn infer(&self, req: InferenceRequest) -> Result<Vec<u8>> {
        let (resp_tx, resp_rx) = oneshot::channel();

        self.tx
            .send_async(BatcherRequest {
                req,
                responder: resp_tx,
            })
            .await
            .map_err(|_| MwvmError::Batching(anyhow::anyhow!("batcher channel closed")))?;

        resp_rx
            .await
            .map_err(|_| MwvmError::Batching(anyhow::anyhow!("batcher response dropped")))?
    }
}

impl Default for ContinuousBatcher {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Background Worker ─────────────────────────────────────────────────────

const MAX_BATCH_SIZE: usize = 32;
const BATCH_TIMEOUT: Duration = Duration::from_millis(8);

async fn worker_loop(rx: Receiver<BatcherRequest>) {
    let mut batch: Vec<BatcherRequest> = Vec::with_capacity(MAX_BATCH_SIZE);
    let mut last_flush = Instant::now();

    loop {
        let remaining = BATCH_TIMEOUT.saturating_sub(last_flush.elapsed());

        tokio::select! {
            result = rx.recv_async() => {
                match result {
                    Ok(req) => {
                        batch.push(req);
                        if batch.len() >= MAX_BATCH_SIZE || last_flush.elapsed() >= BATCH_TIMEOUT {
                            process_batch(&mut batch);
                            last_flush = Instant::now();
                        }
                    }
                    Err(_) => break, // channel closed — shutdown
                }
            }
            () = tokio::time::sleep(remaining) => {
                if !batch.is_empty() {
                    process_batch(&mut batch);
                    last_flush = Instant::now();
                }
            }
        }
    }

    // Drain remaining requests on shutdown.
    for req in batch {
        let _ = req
            .responder
            .send(Err(MwvmError::Batching(anyhow::anyhow!("batcher shutting down"))));
    }
}

/// Process a collected batch of inference requests.
fn process_batch(batch: &mut Vec<BatcherRequest>) {
    if batch.is_empty() {
        return;
    }
    debug!(batch_size = batch.len(), "processing inference batch");

    for req in batch.drain(..) {
        let result = simulate_inference(&req.req);
        if req.responder.send(Ok(result)).is_err() {
            error!("requester dropped before receiving inference result");
        }
    }
}

/// Deterministic inference simulation.
///
/// Generates output proportional to `max_tokens`. In a real deployment this
/// would delegate to a Candle / tract model runner.
fn simulate_inference(req: &InferenceRequest) -> Vec<u8> {
    let output_len = (req.max_tokens as usize).clamp(16, 8192);

    let mut output = Vec::with_capacity(output_len);
    for i in 0..output_len {
        // i % 251 is always in 0..=250, safe to truncate to u8.
        #[allow(clippy::cast_possible_truncation)]
        output.push((i % 251) as u8);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_request(max_tokens: u32) -> InferenceRequest {
        InferenceRequest {
            model_hash: [1u8; 32],
            prompt_hash: [2u8; 32],
            context_root: [0u8; 32],
            max_tokens,
        }
    }

    #[tokio::test]
    async fn single_inference() {
        let batcher = ContinuousBatcher::new();
        let result = batcher.infer(make_request(256)).await;
        assert!(result.is_ok());
        let out = result.unwrap();
        assert!(!out.is_empty());
        assert!(out.len() <= 8192);
    }

    #[tokio::test]
    async fn concurrent_inference() {
        let batcher = ContinuousBatcher::new();
        let mut handles = Vec::new();

        for _ in 0u8..10 {
            let b = batcher.clone();
            handles.push(tokio::spawn(async move {
                b.infer(make_request(64)).await
            }));
        }

        for h in handles {
            assert!(h.await.unwrap().is_ok());
        }
    }
}
