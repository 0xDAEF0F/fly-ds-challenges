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
    let mut msg_id = 1;

    eprintln!("Awaiting messages");

    while let Some(line) = lines.next_line().await? {
        eprintln!("Message received: {}", line);
    }

    Ok(())
}
