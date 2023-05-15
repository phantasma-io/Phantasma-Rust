use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Stake {
    amount: String,
    time: u32,
    unclaimed: String,
}
