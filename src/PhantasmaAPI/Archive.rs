use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Archive {
    name: String,
    hash: String,
    time: u32,
    size: u32,
    encryption: String,
    blockCount: u32,
    missingBlocks: Vec<u32>,
    owner: String,
}
