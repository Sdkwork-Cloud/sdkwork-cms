# SDKWork CMS HTTP Handlers

This support crate owns shared Axum handler state and handler adapters used by
the CMS app, backend, and open API route crates. It does not own a listener or
route manifest; surface-specific paths remain in the canonical route crates.
