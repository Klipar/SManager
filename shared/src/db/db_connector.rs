use sqlx::{postgres::PgPool, Row};

use std::sync::Arc;

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

    pub async fn execute_query(&self, query: &str) -> Result<Vec<Vec<String>>, sqlx::Error> {
        let rows = sqlx::query(query).fetch_all(&*self.pool).await?;
        let mut result = Vec::new();

        for row in rows {
            let mut row_data = Vec::new();
            for i in 0..row.len() {
                let val = match row.try_get::<String, _>(i) {
                    Ok(v) => v,
                    Err(_) => {
                        // fallback for numbers
                        if let Ok(num) = row.try_get::<i32, _>(i) {
                            num.to_string()
                        } else {
                            "".to_string()
                        }
                    }
                };
                row_data.push(val);
            }
            result.push(row_data);
        }

        Ok(result)
    }
}