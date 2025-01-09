use serde::{Deserialize, Serialize};

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "lowercase")]
// pub enum Type {
//     #[serde(rename(serialize = "init_ok"))]
//     Init,
//     #[serde(rename(serialize = "echo_ok"))]
//     Echo,
//     #[serde(rename(serialize = "generate_ok"))]
//     Generate,
// }

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
    #[serde(untagged)]
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
        pub in_reply_to: u32,
        pub msg_id: u32,
        pub echo: String,
    }
}
