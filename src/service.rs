use std::ops::Deref;

use crate::{
    Msg, ServerState,
    client::{ClientMessage, ClientPayload},
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceMsg {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    pub src: String,
    pub dest: String,
    pub body: ServicePayload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServicePayload {
    Read {
        msg_id: u32,
        key: String,
    },
    ReadOk {
        value: String,
        in_reply_to: u32,
    },
    Write {
        msg_id: u32,
        key: String,
        value: String,
    },
    WriteOk {
        in_reply_to: u32,
    },
    Cas {
        msg_id: u32,
        key: String,
        from: String,
        to: String,
    },
    CasOk {
        in_reply_to: u32,
    },
    Error {
        in_reply_to: u32,
        code: u32,
        text: String,
    },
}

impl ServiceMsg {
    pub fn process(self, server_state: &mut ServerState, tx: UnboundedSender<Msg>) {
        match self.body {
            ServicePayload::ReadOk { value, .. } => {
                let num = value.split_once('@').unwrap().0.parse::<u32>().unwrap();

                if num > server_state.counter {
                    server_state.counter = num;
                }

                let next_msg_id = {
                    server_state.msg_id += 1;
                    server_state.msg_id
                };

                let (prev_msg_id, uncommited_delta) = &mut server_state.uncommited_delta;
                *prev_msg_id = next_msg_id;

                let msg = ServiceMsg {
                    id: None,
                    src: server_state.node_id.clone().unwrap(),
                    dest: "seq-kv".to_string(),
                    body: ServicePayload::Cas {
                        msg_id: next_msg_id,
                        key: "counter".to_string(),
                        from: value,
                        to: format!(
                            "{}@{}",
                            num + *uncommited_delta,
                            server_state.node_id.clone().unwrap()
                        ),
                    },
                };
                _ = tx.send(Msg::Service(msg));
            }
            ServicePayload::CasOk { in_reply_to, .. } | ServicePayload::WriteOk { in_reply_to } => {
                let (msg_id, uncommited_delta) = &mut server_state.uncommited_delta;

                if in_reply_to == *msg_id {
                    server_state.counter += *uncommited_delta;
                    *uncommited_delta = 0;
                }

                for (client, msg_id) in server_state.unresponded_msgs.drain() {
                    let msg = ClientMessage {
                        id: None,
                        src: server_state.node_id.clone().unwrap(),
                        dest: client,
                        body: ClientPayload::ReadOk {
                            in_reply_to: msg_id,
                            value: server_state.counter,
                        },
                    };
                    _ = tx.send(Msg::Client(msg));
                }
            }
            ServicePayload::Error { code, .. } => {
                match code {
                    20 => {
                        eprintln!("`key-does-not-exist` error");
                        let msg = ServiceMsg {
                            id: None,
                            src: server_state.node_id.clone().unwrap(),
                            dest: "seq-kv".to_string(),
                            body: ServicePayload::Write {
                                msg_id: server_state.msg_id,
                                key: "counter".to_string(),
                                value: "0@init".to_string(),
                            },
                        };
                        _ = tx.send(Msg::Service(msg));
                    }
                    21 => {
                        eprintln!("`key-already-exists` error");
                    }
                    22 => {
                        // precondition-failed (from value doesn't match)
                        server_state.msg_id += 1;
                        let msg = ServiceMsg {
                            id: None,
                            src: server_state.node_id.clone().unwrap(),
                            dest: "seq-kv".to_string(),
                            body: ServicePayload::Read {
                                msg_id: server_state.msg_id,
                                key: "counter".to_string(),
                            },
                        };
                        _ = tx.send(Msg::Service(msg));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
