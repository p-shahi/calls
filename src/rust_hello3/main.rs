use ic_cdk_macros::*;

#[init]
fn init() {}

#[import(canister = "rust_hello2")]
struct RustHello2;

// Unsupported
// A replicated call cannot call a query which calls more queries
#[update]
async fn update_from_query_from_query() -> Option<u32> {
    RustHello2::query_from_query().await.0
}

// Supported
// A replicated call can call another update which can call a query
#[update]
async fn update_from_update_from_query() -> Option<u32> {
    RustHello2::update_from_query().await.0
}

// Supported
// A replicated call can call another update which can call more updates
#[update]
async fn update_from_update_from_update() -> Option<u32> {
    RustHello2::update_from_update().await.0
}
