use crate::Sheet;
use serde::{Deserialize, Serialize};

// A message from the client (browser) to the server
#[derive(Serialize, Deserialize)]
pub struct ClientMsg {
    pub input: String,
}

// A message from the server to all clients (browsers)
#[derive(Serialize, Deserialize, Clone)]
pub struct ServerMsg {
    pub sheet: Sheet,
}