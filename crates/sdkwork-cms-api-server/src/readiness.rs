use std::future::Future;
use std::pin::Pin;

use sdkwork_web_bootstrap::ReadinessCheck;
use sqlx::PgPool;

pub struct CmsPostgresReadinessCheck {
    pool: PgPool,
}

impl CmsPostgresReadinessCheck {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ReadinessCheck for CmsPostgresReadinessCheck {
    fn check(&self) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + '_>> {
        let pool = self.pool.clone();
        Box::pin(async move {
            sqlx::query("SELECT 1")
                .execute(&pool)
                .await
                .map(|_| ())
                .map_err(|error| error.to_string())
        })
    }
}
