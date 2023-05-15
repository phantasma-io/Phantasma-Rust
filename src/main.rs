use tokio::runtime::Runtime;
pub mod PhantasmaAPI;

fn main() {
    let rt = Runtime::new().unwrap();
    let _result = rt.block_on(PhantasmaAPI::api::get_account("test".to_string()));
}
