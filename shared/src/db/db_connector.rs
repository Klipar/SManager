use sqlx::postgres::PgPool;

use std::sync::Arc;
use sqlx::postgres::PgRow;

pub struct DbConnector {
    pub pool: Arc<PgPool>,
}

impl DbConnector {
    pub async fn new(
        user_var: &str,
        pass_var: &str,
        host_var: &str,
        port_var: &str,
        db_var: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();

        let user = std::env::var(user_var)?;
        let pass = std::env::var(pass_var)?;
        let host = std::env::var(host_var)?;
        let port = std::env::var(port_var)?;
        let db = std::env::var(db_var)?;

        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            user, pass, host, port, db
        );

        let pool = sqlx::postgres::PgPool::connect(&database_url).await?;

        Ok(Self {
            pool: std::sync::Arc::new(pool),
        })
    }

    pub async fn execute_query(&self, query: &str) -> Result<Vec<PgRow>, sqlx::Error> {
        Ok(sqlx::query(query).fetch_all(&*self.pool).await?)
    }
}