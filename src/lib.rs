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

// Parses the string input into a tellraw command to send back to the server with /exec
fn to_json(s: String) -> String {
    unimplemented!();
}

// Send the command to the server with /exec
fn send(s: String) {
    unimplemented!();
}

// Determines if a message is a chat message or not
fn is_chat(s: &String) {
    unimplemented!();
}
