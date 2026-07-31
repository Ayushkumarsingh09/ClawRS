use crate::envelope::EventEnvelope;
use async_trait::async_trait;
use clawrs_core::ClawrsResult;
use std::sync::Arc;
use tokio::sync::broadcast;

const DEFAULT_CAPACITY: usize = 4096;

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle(&self, event: &EventEnvelope) -> ClawrsResult<()>;
}

#[async_trait]
pub trait EventBus: Send + Sync {
    async fn publish(&self, event: EventEnvelope) -> ClawrsResult<()>;
}

pub struct Subscription {
    receiver: broadcast::Receiver<EventEnvelope>,
}

impl Subscription {
    pub async fn recv(&mut self) -> ClawrsResult<EventEnvelope> {
        self.receiver
            .recv()
            .await
            .map_err(|e| clawrs_core::ClawrsError::internal(format!("event bus recv: {e}")))
    }
}

#[derive(Clone)]
pub struct InMemoryEventBus {
    sender: broadcast::Sender<EventEnvelope>,
}

impl Default for InMemoryEventBus {
    fn default() -> Self {
        Self::new(DEFAULT_CAPACITY)
    }
}

impl InMemoryEventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn subscribe(&self) -> Subscription {
        Subscription {
            receiver: self.sender.subscribe(),
        }
    }
}

#[async_trait]
impl EventBus for InMemoryEventBus {
    async fn publish(&self, event: EventEnvelope) -> ClawrsResult<()> {
        let _ = self.sender.send(event);
        Ok(())
    }
}

/// Fan-out handler that delegates to multiple handlers concurrently.
pub struct CompositeHandler {
    handlers: Vec<Arc<dyn EventHandler>>,
}

impl CompositeHandler {
    pub fn new(handlers: Vec<Arc<dyn EventHandler>>) -> Self {
        Self { handlers }
    }
}

#[async_trait]
impl EventHandler for CompositeHandler {
    async fn handle(&self, event: &EventEnvelope) -> ClawrsResult<()> {
        for handler in &self.handlers {
            handler.handle(event).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::envelope::{EventKind, EventEnvelope};
    use clawrs_core::TenantId;

    #[tokio::test]
    async fn publish_subscribe_roundtrip() {
        let bus = InMemoryEventBus::default();
        let mut sub = bus.subscribe();
        let tenant = TenantId::new_v4();
        let event = EventEnvelope::new(
            EventKind::AgentRunStarted,
            tenant,
            serde_json::json!({"hello": "world"}),
        );
        bus.publish(event.clone()).await.unwrap();
        let received = sub.recv().await.unwrap();
        assert_eq!(received.kind, EventKind::AgentRunStarted);
    }
}
