use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub message: String,
    pub username: String,
}
