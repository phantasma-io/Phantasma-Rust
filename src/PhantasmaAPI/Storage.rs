use crate::PhantasmaAPI::Archive::Archive;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Storage {
    available: u64,
    used: u64,
    avatar: String,
    archives: Vec<Archive>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage() {
        let storage = Storage {
            available: 0,
            used: 0,
            avatar: String::from(""),
            archives: vec![],
        };
    }
}
