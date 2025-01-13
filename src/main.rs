use anyhow::Result;
use fly_ds_challenges::{
    client, get_stdin_lines,
    server::{self, ServerState},
};
use std::sync::Arc;
use tokio::{
    sync::Mutex,
    time::{Duration, sleep},
};

#[tokio::main]
async fn main() -> Result<()> {
    eprintln!("Initializing node");

    let server_state = Arc::new(Mutex::new(ServerState::default()));
    let mut lines = get_stdin_lines();

    resend_unack_task(Arc::clone(&server_state));

    eprintln!("Awaiting messages");
    while let Some(line) = lines.next_line().await? {
        eprintln!("Message received: {}", line);

        let client_message = serde_json::from_str::<client::ClientMessage>(&line);
        if let Ok(client_message) = client_message {
            let mut ss = server_state.lock().await;
            client_message
                .process_client_message(&mut ss)
                .send_to_client();
        }

        let inter_node_message = serde_json::from_str::<server::InterNodeMsg>(&line);
        if let Ok(inter_node_message) = inter_node_message {
            let mut ss = server_state.lock().await;
            _ = inter_node_message
                .process_inter_node_msg(&mut ss)
                .map(|response| response.send_to_node());
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
                server_state.send_unack();
            }
            sleep(Duration::from_millis(1_900)).await;
        }
    });
}
