use serde::Deserialize;

#[derive(Deserialize)]
pub struct Ad {
    pub message: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct Auth {
    pub password: String,
    pub user: String,
}

#[derive(Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub scheme: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub rotate: Vec<Ad>,
    pub rpc: Rpc,
}

#[derive(Deserialize)]
pub struct Rpc {
    pub server: Server,
    pub auth: Option<Auth>,
}
