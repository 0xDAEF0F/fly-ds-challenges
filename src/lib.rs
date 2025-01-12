use tokio::io::{AsyncBufReadExt, BufReader, Lines, Stdin, stdin};

pub mod client;
pub mod entity;
pub mod server;

pub fn get_stdin_lines() -> Lines<BufReader<Stdin>> {
    let reader = BufReader::new(stdin());
    reader.lines()
}
