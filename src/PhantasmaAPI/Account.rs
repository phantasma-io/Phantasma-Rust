use crate::PhantasmaAPI::Balance::Balance;
use crate::PhantasmaAPI::Stake::Stake;
use crate::PhantasmaAPI::Storage::Storage;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Account {
    address: String,
    name: String,
    stakes: Stake,
    stake: String,
    unclaimed: String,
    relay: Option<String>,
    validator: String,
    storage: Storage,
    balances: Vec<Balance>,
}
