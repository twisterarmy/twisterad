use serde::Deserialize;

#[derive(Deserialize)]
pub struct Ad {
    pub message: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub rotate: Vec<Ad>,
    pub rpc: Rpc,
}

#[derive(Deserialize)]
pub struct Rpc {
    pub host: String,
    pub password: String,
    pub port: u16,
    pub user: String,
}
