use ic_cdk_macros::*;

#[init]
fn init() {}

#[import(canister = "rust_hello")]
struct RustHello;

// Supported
// Can call a query method from a replicated call
// A replicated can go onto to call one query call
#[update]
async fn update_from_query() -> Option<u32> {
    RustHello::a_query_method().await.0
}

// Supported
// A replicated call can call an update method
#[update]
async fn update_from_update() -> Option<u32> {
    RustHello::an_update_method().await.0
}

// Unsupported
// Cannot call a query method from a non replicated call
#[query]
async fn query_from_query() -> Option<u32> {
    RustHello::a_query_method().await.0
}

// Unsupported
// A query call cannot call an update call
#[query]
async fn query_from_update() -> Option<u32> {
    RustHello::an_update_method().await.0
}
