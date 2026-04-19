use std::sync::Arc;
use sqlx::PgPool;

pub struct AppState {
    pub pool: Arc<PgPool>
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }
}
