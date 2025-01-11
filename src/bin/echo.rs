use anyhow::Result;
use fly_ds_challenges::{client, get_stdin_lines, server::ServerState};

#[tokio::main]
async fn main() -> Result<()> {
    eprintln!("Initializing echo server");

    let mut server_state = ServerState::default();
    let mut lines = get_stdin_lines();

    eprintln!("Awaiting messages");

    while let Some(line) = lines.next_line().await? {
        eprintln!("Message received: {}", line);
        let client_message = serde_json::from_str::<client::ClientMessage>(&line)?;
        let server_messages = client_message.handle_client_message(&mut server_state);

        for server_message in server_messages {
            let server_message_str = serde_json::to_string(&server_message)?;
            println!("{}", server_message_str);
        }
    }

    Ok(())
}
