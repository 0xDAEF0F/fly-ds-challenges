use anyhow::Result;
use fly_ds_challenges::{client, get_stdin_lines, server};

#[tokio::main]
async fn main() -> Result<()> {
    eprintln!("Initializing echo server");

    let mut lines = get_stdin_lines();
    let mut server_state = server::ServerState::default();

    eprintln!("Awaiting messages");

    while let Some(line) = lines.next_line().await? {
        eprintln!("Message received: {}", line);
        let client_message = serde_json::from_str::<client::ClientMessage>(&line)?;
        let server_message = client_message.handle_client_message(&mut server_state);
        let server_message_str = serde_json::to_string(&server_message)?;
        println!("{}", server_message_str);
    }

    Ok(())
}
