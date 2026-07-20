//! Generated gateway bootstrap for sdkwork-cms.
//! Route crates currently expose manifests; assembly stays empty until Router gateway_mount ships.

use axum::Router;

pub struct ApiAssembly {
    pub router: Router,
}

pub fn assemble_api_router() -> ApiAssembly {
    ApiAssembly {
        router: Router::new(),
    }
}
