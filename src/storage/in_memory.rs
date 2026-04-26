use crate::models::MetricSnapshot;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct MetricStore {
    snapshot: Arc<RwLock<MetricSnapshot>>,
}

impl MetricStore {
    pub fn new() -> Self {
        let snapshot = Arc::new(RwLock::new(MetricSnapshot::default()));
        Self { snapshot }
    }

    pub async fn update_snapshot(&self, new_snapshot: MetricSnapshot) {
        let mut snapshot = self.snapshot.write().await;
        *snapshot = new_snapshot;
    }

    pub async fn read_snapshot(&self) -> MetricSnapshot {
        self.snapshot.read().await.clone()
    }
}
