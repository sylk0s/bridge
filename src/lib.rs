use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub servers: Vec<Server>,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub name: String,
    pub send_to: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct McdConf {
    pub ws_port: u16,
}
