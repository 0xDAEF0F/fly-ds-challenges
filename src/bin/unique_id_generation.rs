use anyhow::Result;
use fly_ds_challenges::{client, server};
use tokio::io::{self, AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() -> Result<()> {
    eprintln!("Initializing echo server");

    let stdin = io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    let mut node_id: Option<String> = None;

    eprintln!("Awaiting messages");

    while let Some(line) = lines.next_line().await? {
        eprintln!("Message received: {}", line);
        match serde_json::from_str::<client::ClientMessage>(&line) {
            Ok(client_msg) => match client_msg.body {
                client::ClientBody::Init(init) => {
                    if node_id.is_some() {
                        eprintln!("node id already initialized");
                        continue;
                    }
                    node_id = Some(init.node_id);
                    let server_msg = server::ServerMessage {
                        src: node_id.clone().unwrap(),
                        dest: client_msg.src,
                        body: server::ServerBody::InitOk(server::Init {
                            in_reply_to: init.msg_id,
                        }),
                    };
                    let server_msg_str = serde_json::to_string(&server_msg)?;
                    println!("{}", server_msg_str);
                }
                client::ClientBody::Echo(_) => {}
                client::ClientBody::Generate(generate) => {
                    let unique_id = format!("{}:{}", node_id.clone().unwrap(), generate.msg_id);
                    let body = server::ServerBody::GenerateOk(server::Generate { id: unique_id });
                    let server_msg = server::ServerMessage {
                        src: node_id.clone().unwrap(),
                        dest: client_msg.src,
                        body,
                    };
                    let server_msg_str = serde_json::to_string(&server_msg)?;
                    println!("{}", server_msg_str);
                }
            },
            Err(e) => {
                eprintln!("Unable to deserialize client message: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}
