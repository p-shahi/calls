use ic_cdk_macros::*;

#[query(name = "a_query_method")]
fn get() -> Option<u32> {
    Some(5)
}

#[update(name = "an_update_method")]
fn get_u() -> Option<u32> {
	Some(10)
}
