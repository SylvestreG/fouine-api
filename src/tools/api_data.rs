use crate::errors::FouineApiError;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;

#[derive(Clone)]
pub struct ApiContext {
    pub pool: Arc<PgPool>,
}

impl ApiContext {
    pub async fn new() -> Result<Self, FouineApiError> {
        let database_url = env::var("DATABASE_URL")?;
        let pool = Arc::new(
            PgPoolOptions::new()
                .max_connections(10)
                .connect(&database_url)
                .await
                .map_err(FouineApiError::SqlxError)?,
        );

        Ok(ApiContext { pool })
    }
}
