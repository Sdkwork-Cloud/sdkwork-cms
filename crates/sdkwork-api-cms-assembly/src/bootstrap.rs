//! Generated gateway bootstrap for sdkwork-cms.
//! Route crates currently expose manifests; assembly stays empty until Router gateway_mount ships.

use axum::Router;

pub struct ApplicationAssembly {
    pub router: Router,
}

pub fn assemble_application_router() -> ApplicationAssembly {
    ApplicationAssembly {
        router: Router::new(),
    }
}
