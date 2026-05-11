use std::sync::Arc;
use sqlx::PgPool;
use crate::tls_client::orchestrator::AgentOrchestrator;
pub struct AppState {
    pub pool: Arc<PgPool>,
    pub orchestrator: Arc<AgentOrchestrator>,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: Arc::new(pool),
            orchestrator: Arc::new(AgentOrchestrator::new()),
        }
    }
}