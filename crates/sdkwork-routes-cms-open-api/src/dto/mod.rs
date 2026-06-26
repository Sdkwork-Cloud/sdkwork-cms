//! DTO boundary for CMS open-api route adapters.
//!
//! Open-api DTOs use handwritten mapper inputs.
//! When SDKWork route materialization tooling is available,
//! these can be replaced with generated mapper inputs.

pub mod request;
pub mod response;

pub use request::*;
pub use response::*;
