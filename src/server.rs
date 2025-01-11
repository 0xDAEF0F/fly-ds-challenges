use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct ServerState {
    pub node_id: Option<String>,
    pub msg_id: u32,
    pub messages: HashSet<u32>,
    pub neighbors: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ServerMessage {
    pub src: String,
    pub dest: String,
    pub body: ServerBody,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerBody {
    InitOk(Init),
    EchoOk(Echo),
    GenerateOk(Generate),
    BroadcastOk(Broadcast),
    ReadOk(Read),
    TopologyOk(Topology),
    Whisper(Whisper),
}

#[derive(Debug, Serialize)]
pub struct Init {
    pub in_reply_to: u32,
}

#[derive(Debug, Serialize)]
pub struct Echo {
    pub in_reply_to: u32,
    pub msg_id: u32,
    pub echo: String,
}

#[derive(Debug, Serialize)]
pub struct Generate {
    pub in_reply_to: u32,
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct Broadcast {
    pub in_reply_to: u32,
}

#[derive(Debug, Serialize)]
pub struct Read {
    pub in_reply_to: u32,
    pub messages: Vec<u32>,
}

#[derive(Debug, Serialize)]
pub struct Topology {
    pub in_reply_to: u32,
}

#[derive(Debug, Serialize)]
pub struct Whisper {
    pub message: u32,
}
