pub mod tile;
pub mod tiles;

use dbus_tokio::connection::new_session_sync;
use std::sync::Arc;
use tile::Tile;
use tokio;
use tokio::sync::mpsc::channel;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We can't do much until we have a D-Bus connection so just do it synchronously
    let (resource, conn) = new_session_sync()?;

    // Now start listening on our D-Bus connection
    tokio::spawn(async {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });

    let (mut sender, mut receiver) = channel(1024);

    let instance = Uuid::new_v4().to_string().into_boxed_str();
    let tiles: Vec<Arc<dyn Tile>> = vec![Arc::new(tiles::Time::new(0, sender.clone(), instance))];

    for tile in &tiles {
        tile.clone().spawn();
    }

    let num_tiles = tiles.len();
    tokio::spawn(async move {
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
            eprintln!("Current state: {:?}", blocks);
        }
    });

    loop {}
}
