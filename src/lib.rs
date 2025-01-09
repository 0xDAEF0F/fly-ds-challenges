use serde::{Deserialize, Serialize};

// client -> server
pub mod client {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct ClientMessage {
        pub id: u32,
        pub src: String,
        pub dest: String,
        pub body: ClientBody,
    }

    #[derive(Debug, Deserialize)]
    #[serde(tag = "type", rename_all = "lowercase")]
    pub enum ClientBody {
        Init(Init),
        Echo(Echo),
        Generate(Generate),
    }

    #[derive(Debug, Deserialize)]
    pub struct Init {
        pub msg_id: u32,
        pub node_id: String,
        pub node_ids: Vec<String>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Echo {
        pub msg_id: u32,
        pub echo: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Generate {
        pub msg_id: u32,
    }
}

// server -> client
pub mod server {
    use super::*;

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
        pub id: u32,
    }
}
