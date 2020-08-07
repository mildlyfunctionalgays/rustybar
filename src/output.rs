use crate::config::DefaultSection;
use crate::tile::TileData;
use futures::channel::mpsc::Receiver;
use futures::StreamExt;
use std::convert::Infallible;
use tokio::io::{self, AsyncWriteExt};

pub async fn launch<E>(
    num_tiles: usize,
    mut receiver: Receiver<Result<TileData, E>>,
    _default: DefaultSection,
) -> io::Result<Infallible>
where
    E: Send + std::fmt::Debug,
{
    let mut stdout = io::stdout();
    stdout.write_all(b"{ \"version\": 1 }\n[").await?;

    let mut blocks = Vec::new();
    blocks.resize_with(num_tiles, Default::default);
    loop {
        let message = receiver.next().await.unwrap();
        let message = message.unwrap();
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
