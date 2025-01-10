use crate::server::{self, ServerBody, ServerMessage, ServerState};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ClientMessage {
    pub id: u32,
    pub src: String,
    pub dest: String,
    pub body: ClientBody,
}

impl ClientMessage {
    pub fn handle_client_message(self, server_state: &mut ServerState) -> ServerMessage {
        if !matches!(self.body, ClientBody::Init(_)) {
            if server_state.node_id.is_none() {
                eprintln!("node id not initialized. exiting program.");
                std::process::exit(1);
            }
        }

        let body = match self.body {
            ClientBody::Init(init) => {
                server_state.node_id = Some(init.node_id);
                ServerBody::InitOk(server::Init {
                    in_reply_to: init.msg_id,
                })
            }
            ClientBody::Echo(echo) => {
                server_state.msg_id += 1;
                ServerBody::EchoOk(server::Echo {
                    in_reply_to: echo.msg_id,
                    msg_id: server_state.msg_id,
                    echo: echo.echo,
                })
            }
            ClientBody::Generate(generate) => {
                let unique_id = format!(
                    "{}_{}",
                    server_state.node_id.clone().unwrap(),
                    generate.msg_id
                );
                ServerBody::GenerateOk(server::Generate {
                    id: unique_id,
                    in_reply_to: generate.msg_id,
                })
            }
            ClientBody::Broadcast(broadcast) => {
                server_state.messages.insert(broadcast.message);
                ServerBody::BroadcastOk(server::Broadcast {
                    in_reply_to: broadcast.msg_id,
                })
            }
            ClientBody::Read(read) => ServerBody::ReadOk(server::Read {
                in_reply_to: read.msg_id,
                messages: server_state.messages.iter().cloned().collect(),
            }),
            ClientBody::Topology(mut topology) => {
                let node_id = server_state.node_id.as_ref().unwrap();
                if let Some(neighbors) = topology.topology.remove(node_id) {
                    server_state.neighbors = neighbors;
                }

                ServerBody::TopologyOk(server::Topology {
                    in_reply_to: topology.msg_id,
                })
            }
        };

        ServerMessage {
            src: self.dest,
            dest: self.src,
            body,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ClientBody {
    Init(Init),           // mutates node_id
    Echo(Echo),           // mutates msg_id
    Generate(Generate),   // reads node_id
    Broadcast(Broadcast), // mutates messages
    Read(Read),           // reads messages
    Topology(Topology),   // mutates neighboring nodes
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

#[derive(Debug, Deserialize)]
pub struct Broadcast {
    pub msg_id: u32,
    pub message: u32,
}

#[derive(Debug, Deserialize)]
pub struct Read {
    pub msg_id: u32,
}

#[derive(Debug, Deserialize)]
pub struct Topology {
    pub msg_id: u32,
    pub topology: HashMap<String, Vec<String>>,
}
