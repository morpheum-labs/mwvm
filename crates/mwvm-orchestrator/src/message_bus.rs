//! Topic-based event bus for multi-agent orchestration.
//!
//! Supports targeted agent-to-agent messaging and global broadcasts with
//! high performance and automatic cleanup via RAII subscriptions.

use std::sync::Arc;

use dashmap::DashMap;
use flume::{Receiver, Sender};
use tracing::debug;

use crate::Result;

// =============================================================================
// Event types
// =============================================================================

/// Events flowing through the message bus.
#[derive(Debug, Clone)]
pub enum Event {
    /// Targeted message (or broadcast when `to` is `None`).
    AgentMessage {
        /// Sender agent ID.
        from: u64,
        /// Recipient agent ID (`None` = broadcast).
        to: Option<u64>,
        /// Raw payload bytes.
        payload: Vec<u8>,
    },
    /// System-level lifecycle events.
    System(SystemEvent),
}

/// System events.
#[derive(Debug, Clone)]
pub enum SystemEvent {
    /// Graceful shutdown signal.
    Shutdown,
    /// An agent joined the swarm.
    AgentJoined(/// Agent ID.
        u64),
    /// An agent left the swarm.
    AgentLeft(/// Agent ID.
        u64),
}

/// RAII subscription handle — automatically unsubscribes when dropped.
pub struct Subscription {
    topic: String,
    _sender: Sender<Event>,
    bus: Arc<MessageBusInner>,
}

// =============================================================================
// MessageBus
// =============================================================================

struct MessageBusInner {
    /// Topic → list of subscriber senders.
    topics: DashMap<String, Vec<Sender<Event>>>,
}

/// High-throughput, topic-based message bus.
///
/// Cheap to clone (all state is `Arc`-wrapped).
#[derive(Clone)]
pub struct MessageBus {
    inner: Arc<MessageBusInner>,
}

impl MessageBus {
    /// Create a new empty message bus.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(MessageBusInner {
                topics: DashMap::new(),
            }),
        }
    }

    /// Subscribe to a topic. Returns a subscription handle (RAII) and a receiver.
    pub fn subscribe(&self, topic: impl Into<String>) -> (Subscription, Receiver<Event>) {
        let topic = topic.into();
        let (tx, rx) = flume::unbounded();

        self.inner
            .topics
            .entry(topic.clone())
            .or_default()
            .push(tx.clone());

        let sub = Subscription {
            topic: topic.clone(),
            _sender: tx,
            bus: self.inner.clone(),
        };

        debug!(topic = %topic, "subscriber registered");
        (sub, rx)
    }

    /// Publish an event to all subscribers of `topic`.
    ///
    /// # Errors
    ///
    /// Currently infallible but returns `Result` for forward-compatibility
    /// with persistent transports.
    pub async fn publish(&self, topic: impl Into<String>, event: Event) -> Result<()> {
        let topic = topic.into();

        if let Some(entry) = self.inner.topics.get(&topic) {
            let mut sent = 0usize;
            for sender in entry.value() {
                if sender.send_async(event.clone()).await.is_ok() {
                    sent += 1;
                }
            }
            debug!(topic = %topic, sent_to = sent, "event published");
        } else {
            debug!(topic = %topic, "no subscribers — event dropped");
        }

        Ok(())
    }

    /// Convenience: broadcast to all subscribers of the `"broadcast"` topic.
    ///
    /// # Errors
    ///
    /// Returns an error if the publish operation fails.
    pub async fn broadcast(&self, payload: Vec<u8>) -> Result<()> {
        self.publish(
            "broadcast",
            Event::AgentMessage {
                from: 0,
                to: None,
                payload,
            },
        )
        .await
    }
}

impl Default for MessageBus {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Subscription RAII cleanup
// =============================================================================

impl Drop for Subscription {
    fn drop(&mut self) {
        if let Some(mut entry) = self.bus.topics.get_mut(&self.topic) {
            entry.retain(|tx| !tx.is_disconnected());
        }
        debug!(topic = %self.topic, "subscription dropped");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn publish_subscribe() {
        let bus = MessageBus::new();
        let (_sub, rx) = bus.subscribe("agent:42");

        let event = Event::AgentMessage {
            from: 1,
            to: Some(42),
            payload: b"hello".to_vec(),
        };

        bus.publish("agent:42", event).await.unwrap();

        let received = rx.recv_async().await.unwrap();
        match received {
            Event::AgentMessage { from, payload, .. } => {
                assert_eq!(from, 1);
                assert_eq!(payload, b"hello");
            }
            Event::System(_) => panic!("wrong event type"),
        }
    }

    #[tokio::test]
    async fn broadcast_reaches_all() {
        let bus = MessageBus::new();
        let (_s1, rx1) = bus.subscribe("broadcast");
        let (_s2, rx2) = bus.subscribe("broadcast");

        bus.broadcast(b"hi".to_vec()).await.unwrap();

        let e1 = rx1.recv_async().await.unwrap();
        let e2 = rx2.recv_async().await.unwrap();

        assert!(matches!(e1, Event::AgentMessage { to: None, .. }));
        assert!(matches!(e2, Event::AgentMessage { to: None, .. }));
    }
}
