pub mod client;
pub mod service;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Msg {
    Service(service::ServiceMsg),
    Client(client::ClientMessage),
}

#[derive(Debug, Default)]
pub struct ServerState {
    pub node_id: Option<String>,
    // monotonically increasing for each node
    pub msg_id: u32,
    // grow-only counter challenge
    pub counter: u32,
    pub uncommited_delta: (u32, u32),           // (msg_id, delta)
    pub unresponded_msgs: HashMap<String, u32>, // client => msg_id
}
