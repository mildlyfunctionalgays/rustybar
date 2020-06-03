use crate::tile::TileData;
use tokio::io::{self, AsyncWriteExt};
use tokio::sync::mpsc::Receiver;
use std::convert::Infallible;

pub async fn launch(num_tiles: usize, mut receiver: Receiver<TileData>) -> io::Result<Infallible> {
    let mut stdout = io::stdout();
    stdout.write_all(b"{ \"version\": 1 }\n[").await?;

    let mut blocks = Vec::new();
    blocks.resize_with(num_tiles, Default::default);
    loop {
        let message = receiver.recv().await.unwrap();
        if message.sender_id < num_tiles {
            blocks[message.sender_id] = Some(message.block);
        } else {
            eprintln!("Invalid message with sender id {}", message.sender_id);
            continue;
        }
        let serialized = serde_json::to_vec(&blocks).unwrap();
        stdout.write_all(&serialized).await?;
        stdout.write_all(b",\n").await?;
    }
}
