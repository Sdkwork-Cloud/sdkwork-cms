//! SQLx repository implementation for SDKWork CMS.
//!
//! This crate implements the `CmsRepository` port using SQLx with PostgreSQL.
//! Includes pool management, migration runner, row mapping, and repository queries.

pub mod db;
pub mod error;
pub mod mapper;
pub mod repository;

pub use error::{CmsRepositoryError, CmsRepositoryResult};
pub use repository::CmsSqlxRepository;
