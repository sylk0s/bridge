use bridge_app::{Config, McdConf};
use std::fs;
use futures::StreamExt;

const CONF_PATH: &str = "/home/sylkos/.config/mc-docker";

#[tokio::main]
async fn main() {
    let mcd_file = fs::read_to_string(format!("{CONF_PATH}/config.toml")).expect("Failed to read mcd config"); 
    let mcd_config: McdConf = toml::from_str(&mcd_file).expect("Could not parse config.toml");
    
    let config_file = fs::read_to_string(format!("{CONF_PATH}/bridge.json")).expect("Failed to read bridge.json");
    let config: Config = serde_json::from_str(&config_file).expect("Could not parse bridge.json");

    let path = format!("localhost:{}", mcd_config.ws_port);

    for server in config.servers {
        let p = path.clone();
        tokio::spawn(async move {
            let mut stream = reqwest::get(format!("{p}/cleanout/{}", server.name)).await.unwrap().bytes_stream();
            while let Some(msg) = stream.next().await {
                let msg = std::str::from_utf8(&msg.unwrap()).unwrap().to_string();
                for destination in &server.send_to {
                    let body = format!(r#"{{"args"["tellraw", "@a" "\"{{\"text\":\"{}\"}}\""]}}"#, msg);
                reqwest::Client::new().post(format!("{p}/exec/{}", destination)).body(body)
                    .send().await.expect("Failed to send command to server");
                }
            }
        });
    }
}
