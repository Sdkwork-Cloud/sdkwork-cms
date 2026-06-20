//! SQLx repository implementation for SDKWork CMS.
//!
//! This crate implements the `CmsRepository` port using SQLx with PostgreSQL.
//! Includes pool management, migration runner, row mapping, and repository queries.

mod bootstrap;

pub mod db;
pub mod error;
pub mod mapper;
pub mod repository;

pub use bootstrap::{
    bootstrap_cms_database, bootstrap_cms_database_from_env, connect_and_bootstrap_cms_database_from_env,
    connect_cms_database_pool_from_env, CmsDatabaseHost, CmsDatabasePool,
};
pub use error::{CmsRepositoryError, CmsRepositoryResult};
pub use repository::CmsSqlxRepository;
