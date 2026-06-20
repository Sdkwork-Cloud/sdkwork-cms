//! SDKWork CMS database pool bootstrap via `sdkwork-database`.

use sdkwork_database_config::DatabaseConfig;
use sdkwork_database_sqlx::{create_pool_from_config, DatabasePool, PoolError};

pub use sdkwork_cms_database_host::{
    bootstrap_cms_database, bootstrap_cms_database_from_env, CmsDatabaseHost,
};

pub type CmsDatabasePool = DatabasePool;

pub async fn connect_cms_database_pool_from_env() -> Result<CmsDatabasePool, PoolError> {
    let config = DatabaseConfig::from_env("CMS")?;
    create_pool_from_config(config).await
}

pub async fn connect_and_bootstrap_cms_database_from_env() -> Result<CmsDatabaseHost, String> {
    let pool = connect_cms_database_pool_from_env()
        .await
        .map_err(|error| error.to_string())?;
    bootstrap_cms_database(pool).await
}
