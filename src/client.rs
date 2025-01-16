use crate::{
    Msg, ServerState,
    service::{ServiceMsg, ServicePayload},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    pub src: String,
    pub dest: String,
    pub body: ClientPayload,
}

impl ClientMessage {
    pub fn process(self, server_state: &mut ServerState, tx: UnboundedSender<Msg>) {
        match self.body {
            ClientPayload::Init {
                node_id, msg_id, ..
            } => {
                server_state.node_id = Some(node_id.clone());
                let msg = ClientMessage {
                    id: None,
                    src: node_id,
                    dest: self.src,
                    body: ClientPayload::InitOk {
                        in_reply_to: msg_id,
                    },
                };
                _ = tx.send(Msg::Client(msg));
            }
            ClientPayload::Topology { msg_id, .. } => {
                let msg = ClientMessage {
                    id: None,
                    src: server_state.node_id.clone().unwrap(),
                    dest: self.src,
                    body: ClientPayload::TopologyOk {
                        in_reply_to: msg_id,
                    },
                };
                _ = tx.send(Msg::Client(msg));
            }
            ClientPayload::Read { msg_id } => {
                let msg = ClientMessage {
                    id: None,
                    src: server_state.node_id.clone().unwrap(),
                    dest: self.src,
                    body: ClientPayload::ReadOk {
                        in_reply_to: msg_id,
                        value: server_state.last_seen_counter
                            + server_state.uncommited_deltas.values().sum::<u32>(),
                    },
                };
                _ = tx.send(Msg::Client(msg));

                server_state.msg_id += 1;
                let msg = ServiceMsg {
                    src: server_state.node_id.clone().unwrap(),
                    dest: "seq-kv".to_string(),
                    body: ServicePayload::Read {
                        msg_id: server_state.msg_id,
                        key: "counter".to_string(),
                    },
                };
                _ = tx.send(Msg::Service(msg));
            }
            ClientPayload::Add { delta, msg_id } => {
                let msg = ClientMessage {
                    id: None,
                    src: server_state.node_id.clone().unwrap(),
                    dest: self.src,
                    body: ClientPayload::AddOk {
                        in_reply_to: msg_id,
                    },
                };
                _ = tx.send(Msg::Client(msg));

                server_state.msg_id += 1;
                let msg_id = server_state.msg_id;
                server_state.uncommited_deltas.insert(msg_id, delta);

                let service_payload = if server_state.last_seen_counter == 0 {
                    ServicePayload::Write {
                        msg_id,
                        key: "counter".to_string(),
                        value: delta,
                    }
                } else {
                    ServicePayload::Cas {
                        msg_id,
                        key: "counter".to_string(),
                        from: server_state.last_seen_counter,
                        to: server_state.last_seen_counter + delta,
                    }
                };
                let msg = ServiceMsg {
                    src: server_state.node_id.clone().unwrap(),
                    dest: "seq-kv".to_string(),
                    body: service_payload,
                };
                _ = tx.send(Msg::Service(msg));
            }
            _ => {}
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientPayload {
    Init {
        msg_id: u32,
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {
        in_reply_to: u32,
    },
    Topology {
        msg_id: u32,
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk {
        in_reply_to: u32,
    },
    Read {
        msg_id: u32,
    },
    ReadOk {
        in_reply_to: u32,
        value: u32,
    },
    Add {
        msg_id: u32,
        delta: u32,
    },
    AddOk {
        in_reply_to: u32,
    },
}
