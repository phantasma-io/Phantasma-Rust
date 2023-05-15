use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ErrorHandler {
    error: Option<String>,
}
