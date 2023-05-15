use crate::PhantasmaAPI::api;
use tokio::runtime::Runtime;
pub mod PhantasmaAPI;

fn main() {
    let mut rt = Runtime::new().unwrap();
    let result = rt.block_on(PhantasmaAPI::api::getAccount(
        "P2KCH6yqtj9DYchypeY7BrUrhiCKpP59UgisgijtXjDwcLp".to_string(),
    ));
}
