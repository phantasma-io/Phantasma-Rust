use crate::PhantasmaAPI::Archive::Archive;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Storage {
    available: u64,
    used: u64,
    avatar: String,
    archives: Vec<Archive>,
}
