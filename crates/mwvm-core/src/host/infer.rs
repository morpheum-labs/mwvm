//! Rich local inference host — `morpheum_infer`.
//!
//! Zero-copy read of [`InferenceRequest`] from WASM linear memory, delegation
//! to [`ContinuousBatcher`] when available, and zero-copy write-back.

use super::prelude::*;

/// Register `morpheum_infer` with the wasmtime [`Linker`].
///
/// # Errors
///
/// Returns [`MwvmError::HostRegistration`] if wasmtime rejects the binding.
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn register(linker: &mut Linker<StoreContext>, registry: &HostRegistry) -> Result<()> {
    let reg = registry.clone();
    linker
        .func_wrap(
            "morpheum",
            "morpheum_infer",
            move |mut caller: Caller<'_, StoreContext>,
                  req_ptr: i32,
                  req_len: i32,
                  out_ptr: i32,
                  out_max_len: i32|
                  -> anyhow::Result<i32> {
                let mem = caller
                    .get_export("memory")
                    .and_then(wasmtime::Extern::into_memory)
                    .ok_or_else(|| anyhow::anyhow!("memory export not found"))?;

                // ── Read request (scoped borrow) ──
                let req: InferenceRequest = {
                    let data = mem.data(&caller);
                    let start = req_ptr as usize;
                    let end = start + req_len as usize;
                    let slice = data
                        .get(start..end)
                        .ok_or_else(|| anyhow::anyhow!("invalid request ptr/len"))?;
                    *bytemuck::from_bytes(slice)
                };

                req.validate()
                    .map_err(|e| anyhow::anyhow!("validation: {e}"))?;

                // ── Delegate to batcher ──
                let batcher = reg
                    .batcher
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("model-serving not enabled"))?;

                // Use tokio handle for blocking bridge — host funcs are sync.
                let output = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(batcher.infer(req))
                })
                .map_err(|e| anyhow::anyhow!("inference failed: {e}"))?;

                // ── Write result back ──
                {
                    let data = mem.data_mut(&mut caller);
                    let start = out_ptr as usize;
                    let max = out_max_len as usize;
                    let dest = data
                        .get_mut(start..start + max)
                        .ok_or_else(|| anyhow::anyhow!("output buffer too small"))?;

                    let copy_len = output.len().min(dest.len());
                    dest[..copy_len].copy_from_slice(&output[..copy_len]);
                    Ok(copy_len as i32)
                }
            },
        )
        .map_err(|e| MwvmError::HostRegistration {
            name: "morpheum_infer",
            source: e,
        })?;

    Ok(())
}
