#![allow(dead_code)]

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::io::{self, AsyncBufReadExt, BufReader};

#[derive(Debug, Deserialize)]
enum ClientMessage {
    Init(Init),
    Echo(Echo),
}

#[derive(Debug, Deserialize)]
struct Init {
    r#type: String,
    msg_id: u32,
    node_id: String,
    node_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Echo {
    src: String,
    dest: String,
    body: Body,
}

#[derive(Debug, Deserialize, Serialize)]
struct Body {
    r#type: String,
    in_reply_to: Option<u32>,
    msg_id: u32,
    echo: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    eprintln!("Initializing echo server");

    let stdin = io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    let mut node_id: Option<String> = None;
    let mut msg_id = 1;

    eprintln!("Awaiting messages");

    while let Some(line) = lines.next_line().await? {
        eprintln!("Message received. Deserializing...");
        match serde_json::from_str::<ClientMessage>(&line) {
            Ok(client_msg) => match client_msg {
                ClientMessage::Init(init) => {
                    node_id = Some(init.node_id);
                    let init_reply = serde_json::json!({
                        "type": "init_ok",
                        "in_reply_to": init.msg_id
                    });
                    println!("{}", init_reply.to_string());
                }
                ClientMessage::Echo(echo) => {
                    if node_id.as_ref().is_some_and(|id| id == echo.dest.as_str()) {
                        // reply back
                        let echo = Echo {
                            src: node_id.clone().unwrap(),
                            dest: echo.src,
                            body: Body {
                                r#type: "echo".to_string(),
                                in_reply_to: Some(echo.body.msg_id),
                                msg_id: msg_id,
                                echo: echo.body.echo,
                            },
                        };
                        match serde_json::to_string(&echo) {
                            Ok(echo_str) => {
                                msg_id += 1;
                                println!("{}", echo_str);
                            }
                            Err(e) => {
                                eprintln!("Error serializing echo message: {:?}", e);
                                eprintln!("msg: {:?}", line);
                                break;
                            }
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("Error deserializing client message: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}
