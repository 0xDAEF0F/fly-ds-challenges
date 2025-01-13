use crate::entity::{Client, Node};
use crate::server::{self, ServerState, ServerToClientBody, ServerToClientMsg};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Deserialize)]
pub struct ClientMessage {
    pub id: u32,
    pub src: Client,
    pub dest: Node,
    pub body: ClientBody,
}

impl ClientMessage {
    pub fn process_client_message(self, server_state: &mut ServerState) -> ServerToClientMsg {
        if !matches!(self.body, ClientBody::Init(_)) && server_state.node_id.is_none() {
            eprintln!("node id not initialized. exiting program.");
            std::process::exit(1);
        }

        let body = match self.body {
            ClientBody::Init(init) => {
                server_state.node_ids = init
                    .node_ids
                    .into_iter()
                    .filter(|n| n != &init.node_id)
                    .collect();
                server_state.node_id = Some(init.node_id);

                ServerToClientBody::InitOk(server::Init {
                    in_reply_to: init.msg_id,
                })
            }
            ClientBody::Echo(echo) => {
                server_state.msg_id += 1;

                ServerToClientBody::EchoOk(server::Echo {
                    in_reply_to: echo.msg_id,
                    msg_id: server_state.msg_id,
                    echo: echo.echo,
                })
            }
            ClientBody::Generate(generate) => {
                let ss = server_state.node_id.unwrap();
                let unique_id = format!(
                    "{}_{}",
                    server_state.node_id.unwrap().to_string(),
                    generate.msg_id
                );

                ServerToClientBody::GenerateOk(server::Generate {
                    id: unique_id,
                    in_reply_to: generate.msg_id,
                })
            }
            ClientBody::Broadcast(broadcast) => {
                server_state.messages.insert(broadcast.message);
                for node in &server_state.node_ids {
                    server_state
                        .unack_neigh_msgs
                        .entry(node.clone())
                        .or_insert_with(HashSet::new)
                        .insert(broadcast.message);
                }

                ServerToClientBody::BroadcastOk(server::Broadcast {
                    in_reply_to: broadcast.msg_id,
                })
            }
            ClientBody::Read(read) => {
                server_state.send_unack();

                ServerToClientBody::ReadOk(server::Read {
                    in_reply_to: read.msg_id,
                    messages: server_state.messages.iter().cloned().collect(),
                })
            }
            ClientBody::Topology(mut topology) => {
                let node_id = server_state.node_id.as_ref().unwrap();
                if let Some(neighbors) = topology.topology.remove(node_id) {
                    server_state.neighbors = neighbors;
                }

                ServerToClientBody::TopologyOk(server::Topology {
                    in_reply_to: topology.msg_id,
                })
            }
        };

        ServerToClientMsg {
            src: self.dest,
            dest: self.src,
            body,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientBody {
    Init(Init),
    Echo(Echo),
    Generate(Generate),
    Broadcast(Broadcast),
    Read(Read),
    Topology(Topology),
}

#[derive(Debug, Deserialize)]
pub struct Init {
    pub msg_id: u32,
    pub node_id: Node,
    pub node_ids: Vec<Node>,
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
    pub topology: HashMap<Node, Vec<Node>>,
}
