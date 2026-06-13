use sqlx::PgPool;

use crate::error::{CmsRepositoryError, CmsRepositoryResult};

#[derive(Clone)]
pub struct CmsSqlxRepository {
    pool: PgPool,
}

impl CmsSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub(crate) fn generate_id(&self) -> i64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock went backwards");
        let millis = duration.as_millis() as i64;
        let random_part = (uuid::Uuid::new_v4().as_u128() & 0xFFF) as i64;
        (millis << 22) | (random_part & 0x3FFFFF)
    }

    pub(crate) fn generate_uuid(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }

    pub(crate) fn current_timestamp(&self) -> String {
        chrono::Utc::now().to_rfc3339()
    }

    pub async fn run_migrations(&self) -> CmsRepositoryResult<()> {
        let migration_sql =
            include_str!("../../migrations/0001_cms_v1_foundation.sql");
        sqlx::raw_sql(migration_sql)
            .execute(&self.pool)
            .await
            .map_err(|e| CmsRepositoryError::Database(e.to_string()))?;
        Ok(())
    }
}

impl From<sqlx::Error> for CmsRepositoryError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => CmsRepositoryError::NotFound("row not found"),
            _ => CmsRepositoryError::Database(err.to_string()),
        }
    }
}
