use crate::envelope::EventEnvelope;
use async_trait::async_trait;
use clawrs_core::{ClawrsResult, TenantId};
use parking_lot::RwLock;
use std::collections::HashMap;

#[async_trait]
pub trait EventStore: Send + Sync {
    async fn append(&self, event: EventEnvelope) -> ClawrsResult<()>;
    async fn load(&self, tenant_id: TenantId, limit: usize) -> ClawrsResult<Vec<EventEnvelope>>;
}

pub struct InMemoryEventStore {
    events: RwLock<Vec<EventEnvelope>>,
}

impl Default for InMemoryEventStore {
    fn default() -> Self {
        Self {
            events: RwLock::new(Vec::new()),
        }
    }
}

#[async_trait]
impl EventStore for InMemoryEventStore {
    async fn append(&self, event: EventEnvelope) -> ClawrsResult<()> {
        self.events.write().push(event);
        Ok(())
    }

    async fn load(&self, tenant_id: TenantId, limit: usize) -> ClawrsResult<Vec<EventEnvelope>> {
        let events = self.events.read();
        let filtered: Vec<_> = events
            .iter()
            .filter(|e| e.tenant_id == tenant_id)
            .rev()
            .take(limit)
            .cloned()
            .collect();
        Ok(filtered.into_iter().rev().collect())
    }
}

/// Simple projection index keyed by tenant (extensible to CQRS read models).
pub struct TenantProjectionIndex {
    counts: RwLock<HashMap<TenantId, u64>>,
}

impl Default for TenantProjectionIndex {
    fn default() -> Self {
        Self {
            counts: RwLock::new(HashMap::new()),
        }
    }
}

impl TenantProjectionIndex {
    pub fn increment(&self, tenant_id: TenantId) {
        let mut map = self.counts.write();
        *map.entry(tenant_id).or_insert(0) += 1;
    }

    pub fn count(&self, tenant_id: TenantId) -> u64 {
        self.counts.read().get(&tenant_id).copied().unwrap_or(0)
    }
}

pub struct ProjectingEventHandler<S: EventStore> {
    store: S,
    index: TenantProjectionIndex,
}

impl<S: EventStore> ProjectingEventHandler<S> {
    pub fn new(store: S, index: TenantProjectionIndex) -> Self {
        Self { store, index }
    }
}

#[async_trait]
impl<S: EventStore + Sync> crate::bus::EventHandler for ProjectingEventHandler<S> {
    async fn handle(&self, event: &EventEnvelope) -> ClawrsResult<()> {
        self.index.increment(event.tenant_id);
        self.store.append(event.clone()).await
    }
}
