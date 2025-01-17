use crate::{Msg, ServerState};
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
        value: u32,
        in_reply_to: u32,
    },
    Write {
        msg_id: u32,
        key: String,
        value: u32,
    },
    WriteOk {
        in_reply_to: u32,
    },
    Cas {
        msg_id: u32,
        key: String,
        from: u32,
        to: u32,
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
                if value >= server_state.last_seen_counter {
                    server_state.last_seen_counter = value;
                } else {
                    panic!("`ReadOk` value is less than `last_seen_counter`");
                }

                // retry uncommited deltas
                if !server_state.uncommited_deltas.is_empty() {
                    let acc_deltas = server_state
                        .uncommited_deltas
                        .drain()
                        .map(|(_, d)| d)
                        .sum::<u32>();
                    server_state.msg_id += 1;
                    server_state
                        .uncommited_deltas
                        .insert(server_state.msg_id, acc_deltas);
                    let msg = ServiceMsg {
                        id: None,
                        src: server_state.node_id.clone().unwrap(),
                        dest: "seq-kv".to_string(),
                        body: ServicePayload::Cas {
                            msg_id: server_state.msg_id,
                            key: "counter".to_string(),
                            from: server_state.last_seen_counter,
                            to: server_state.last_seen_counter + acc_deltas,
                        },
                    };
                    _ = tx.send(Msg::Service(msg));
                } else {
                    server_state.msg_id += 1;
                    let msg = ServiceMsg {
                        id: None,
                        src: server_state.node_id.clone().unwrap(),
                        dest: "seq-kv".to_string(),
                        body: ServicePayload::Cas {
                            msg_id: server_state.msg_id,
                            key: "counter".to_string(),
                            from: server_state.last_seen_counter,
                            to: server_state.last_seen_counter,
                        },
                    };
                    _ = tx.send(Msg::Service(msg));
                }
            }
            ServicePayload::WriteOk { .. } => {}
            ServicePayload::CasOk { in_reply_to, .. } => {
                if let Some(delta) = server_state.uncommited_deltas.remove(&in_reply_to) {
                    server_state.last_seen_counter += delta;
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
                                value: 0,
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
