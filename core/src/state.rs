use std::sync::Arc;
use sqlx::PgPool;
use tokio::sync::broadcast;

use crate::tls_client::orchestrator::AgentOrchestrator;
use shared::db::models::run::Run;

#[derive(Clone, Debug, serde::Serialize)]
pub struct RunEvent {
    pub agent_id: i64,
    pub run: Run,
}

pub struct AppState {
    pub pool: Arc<PgPool>,
    pub orchestrator: Arc<AgentOrchestrator>,
    pub run_tx: broadcast::Sender<RunEvent>,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        let (run_tx, _) = broadcast::channel(1024);
        let pool = Arc::new(pool);

        Self {
            pool: Arc::clone(&pool),
            orchestrator: Arc::new(AgentOrchestrator::new(run_tx.clone(), pool)),
            run_tx,
        }
    }
}