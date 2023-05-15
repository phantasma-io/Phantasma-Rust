use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Balance {
    chain: String,
    amount: String,
    symbol: String,
    decimals: u32,
    ids: Vec<String>,
}
