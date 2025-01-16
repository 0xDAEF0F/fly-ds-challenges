use anyhow::Result;
use fly_ds_challenges::{Msg, ServerState};
use std::sync::Arc;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    sync::{Mutex, mpsc},
};

#[tokio::main]
async fn main() -> Result<()> {
    eprintln!("Initializing node");

    let server_state = Arc::new(Mutex::new(ServerState::default()));

    let reader = BufReader::new(tokio::io::stdin());
    let mut lines = reader.lines();

    let (tx, mut rx) = mpsc::unbounded_channel::<Msg>();

    tokio::spawn(async move {
        eprintln!("Starting stdout task");
        while let Some(msg) = rx.recv().await {
            println!("{}", serde_json::to_string(&msg).unwrap());
        }
    });

    eprintln!("Awaiting messages");
    while let Some(line) = lines.next_line().await? {
        eprintln!("Message received: {}", line);

        let msg: Msg = serde_json::from_str(&line)?;
        let mut server_state = server_state.lock().await;
        match msg {
            Msg::Client(client_msg) => {
                client_msg.process(&mut server_state, tx.clone());
            }
            Msg::Service(service_msg) => {
                service_msg.process(&mut server_state, tx.clone());
            }
        }
    }

    Ok(())
}
