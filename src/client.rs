use crate::server::{self, ServerBody, ServerMessage, ServerState};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Deserialize)]
pub struct ClientMessage {
    pub id: u32,
    pub src: String,
    pub dest: String,
    pub body: ClientBody,
}

impl ClientMessage {
    pub fn handle_client_message(
        self,
        server_state: &mut ServerState,
    ) -> impl Iterator<Item = ServerMessage> {
        if !matches!(self.body, ClientBody::Init(_)) && server_state.node_id.is_none() {
            eprintln!("node id not initialized. exiting program.");
            std::process::exit(1);
        }

        let whispers = if let ClientBody::Broadcast(broadcast) = &self.body {
            Some(server_state.node_ids.iter().cloned().map(|n| {
                server_state
                    .unack_neigh_msgs
                    .entry(n.clone())
                    .or_insert_with(HashSet::new)
                    .insert(broadcast.message);
                ServerMessage {
                    dest: n,
                    src: server_state.node_id.as_ref().unwrap().clone(),
                    body: ServerBody::Whisper(server::Whisper {
                        messages: vec![broadcast.message],
                    }),
                }
            }))
        } else {
            None
        };
        let whispers: Vec<_> = whispers.into_iter().flatten().collect();

        let body = match self.body {
            ClientBody::Init(init) => {
                server_state.node_ids = init
                    .node_ids
                    .into_iter()
                    .filter(|n| n != &init.node_id)
                    .collect();
                server_state.node_id = Some(init.node_id);
                Some(ServerBody::InitOk(server::Init {
                    in_reply_to: init.msg_id,
                }))
            }
            ClientBody::Echo(echo) => {
                server_state.msg_id += 1;
                Some(ServerBody::EchoOk(server::Echo {
                    in_reply_to: echo.msg_id,
                    msg_id: server_state.msg_id,
                    echo: echo.echo,
                }))
            }
            ClientBody::Generate(generate) => {
                let unique_id = format!(
                    "{}_{}",
                    server_state.node_id.clone().unwrap(),
                    generate.msg_id
                );
                Some(ServerBody::GenerateOk(server::Generate {
                    id: unique_id,
                    in_reply_to: generate.msg_id,
                }))
            }
            ClientBody::Broadcast(broadcast) => {
                server_state.messages.insert(broadcast.message);
                Some(ServerBody::BroadcastOk(server::Broadcast {
                    in_reply_to: broadcast.msg_id,
                }))
            }
            ClientBody::Read(read) => {
                server_state.resend_unack();
                Some(ServerBody::ReadOk(server::Read {
                    in_reply_to: read.msg_id,
                    messages: server_state.messages.iter().cloned().collect(),
                }))
            }
            ClientBody::Topology(mut topology) => {
                let node_id = server_state.node_id.as_ref().unwrap();
                if let Some(neighbors) = topology.topology.remove(node_id) {
                    server_state.neighbors = neighbors;
                }

                Some(ServerBody::TopologyOk(server::Topology {
                    in_reply_to: topology.msg_id,
                }))
            }
            ClientBody::Whisper(whisper) => {
                for &msg in &whisper.messages {
                    server_state.messages.insert(msg);
                }

                Some(ServerBody::WhisperOk(server::Whisper {
                    messages: whisper.messages,
                }))
            }
            ClientBody::WhisperOk(whisper) => {
                server_state
                    .unack_neigh_msgs
                    .entry(self.src.clone())
                    .and_modify(|msgs| {
                        for msg in &whisper.messages {
                            msgs.remove(msg);
                        }
                    });

                None
            }
        };

        let server_message = body.map(|b| ServerMessage {
            src: self.dest,
            dest: self.src,
            body: b,
        });

        whispers
            .into_iter()
            .chain(std::iter::once(server_message).filter_map(|x| x))
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
    Whisper(Whisper),
    WhisperOk(Whisper),
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

#[derive(Debug, Deserialize)]
pub struct Whisper {
    pub messages: Vec<u32>,
}
