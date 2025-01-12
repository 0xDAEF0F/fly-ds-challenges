use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub struct ServerState {
    pub node_id: Option<String>,
    pub msg_id: u32,
    pub messages: HashSet<u32>,
    pub neighbors: Vec<String>,
    pub node_ids: Vec<String>,
    pub unack_neigh_msgs: HashMap<String, HashSet<u32>>,
}

impl ServerState {
    pub fn send_unack(&self) {
        self.unack_neigh_msgs.iter().for_each(|(n, msgs)| {
            if msgs.is_empty() {
                return;
            }
            println!(
                "{}",
                serde_json::to_string(&InterNodeMsg {
                    src: self.node_id.as_ref().unwrap().clone(),
                    dest: n.clone(),
                    body: InterNodeBody::Whisper(Whisper {
                        messages: msgs.iter().cloned().collect()
                    }),
                })
                .unwrap()
            );
        });
    }
}

#[derive(Debug, Serialize)]
pub struct ServerToClientMsg {
    pub src: String,
    pub dest: String,
    pub body: ServerToClientBody,
}

impl ServerToClientMsg {
    pub fn send_to_client(&self) {
        let serialized_msg = serde_json::to_string(self).unwrap();
        println!("{}", serialized_msg);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InterNodeMsg {
    pub src: String,
    pub dest: String,
    pub body: InterNodeBody,
}

impl InterNodeMsg {
    pub fn send_to_node(&self) {
        let serialized_msg = serde_json::to_string(self).unwrap();
        println!("{}", serialized_msg);
    }
}

impl InterNodeMsg {
    pub fn send_inter_node_msg_to_client(msg: Option<&InterNodeMsg>) {
        if let Some(msg) = msg {
            let serialized_msg = serde_json::to_string(msg).unwrap();
            println!("{}", serialized_msg);
        }
    }

    pub fn process_inter_node_msg(self, server_state: &mut ServerState) -> Option<InterNodeMsg> {
        match self.body {
            InterNodeBody::WhisperOk(whisper) => {
                if let Some(hs) = server_state.unack_neigh_msgs.get_mut(&self.src) {
                    hs.retain(|n| !whisper.messages.contains(n));
                }

                None
            }
            InterNodeBody::Whisper(whisper) => {
                server_state
                    .messages
                    .extend(whisper.messages.iter().cloned());

                Some(InterNodeMsg {
                    src: server_state.node_id.as_ref().unwrap().clone(),
                    dest: self.src,
                    body: InterNodeBody::WhisperOk(Whisper {
                        messages: whisper.messages,
                    }),
                })
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InterNodeBody {
    WhisperOk(Whisper),
    Whisper(Whisper),
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerToClientBody {
    InitOk(Init),
    EchoOk(Echo),
    GenerateOk(Generate),
    BroadcastOk(Broadcast),
    ReadOk(Read),
    TopologyOk(Topology),
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Whisper {
    pub messages: Vec<u32>,
}
