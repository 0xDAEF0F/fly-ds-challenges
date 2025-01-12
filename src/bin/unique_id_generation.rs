use anyhow::Result;
use fly_ds_challenges::{client, get_stdin_lines, server::ServerState};
use std::sync::Arc;
use tokio::{
    sync::Mutex,
    time::{Duration, sleep},
};

#[tokio::main]
async fn main() -> Result<()> {
    eprintln!("Initializing unique id generation server");

    let server_state = Arc::new(Mutex::new(ServerState::default()));
    let mut lines = get_stdin_lines();

    resend_unack_task(Arc::clone(&server_state));

    eprintln!("Awaiting messages");
    while let Some(line) = lines.next_line().await? {
        eprintln!("Message received: {}", line);

        let client_message = serde_json::from_str::<client::ClientMessage>(&line)?;
        let mut server_state = server_state.lock().await;
        let server_messages = client_message
            .handle_client_message(&mut server_state)
            .into_iter();

        for server_message in server_messages {
            let server_message_str = serde_json::to_string(&server_message)?;
            println!("{}", server_message_str);
        }
    }

    Ok(())
}

fn resend_unack_task(server_state: Arc<Mutex<ServerState>>) {
    eprintln!("Starting resend unack task");
    tokio::spawn(async move {
        loop {
            {
                let server_state = server_state.lock().await;
                server_state.resend_unack();
            }
            sleep(Duration::from_secs(1)).await;
        }
    });
}
