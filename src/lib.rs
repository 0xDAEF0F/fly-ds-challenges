use serde::{Deserialize, Serialize};

// client -> server
pub mod client {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct ClientMessage {
        pub src: String,
        pub dest: String,
        pub body: ClientBody,
    }

    #[derive(Debug, Deserialize)]
    pub enum ClientBody {
        Init(Init),
        Echo(Echo),
    }

    #[derive(Debug, Deserialize)]
    pub struct Init {
        pub r#type: String,
        pub msg_id: u32,
        pub node_id: String,
        pub node_ids: Vec<String>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Echo {
        pub r#type: String,
        pub msg_id: u32,
        pub echo: String,
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
    pub enum ServerBody {
        Init(Init),
        Echo(Echo),
    }

    #[derive(Debug, Serialize)]
    pub struct Init {
        pub r#type: String,
        pub in_reply_to: u32,
    }

    #[derive(Debug, Serialize)]
    pub struct Echo {
        pub r#type: String,
        pub msg_id: u32,
        pub in_reply_to: u32,
        pub echo: String,
    }
}
