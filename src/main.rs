pub mod tile;
pub mod tiles;

use dbus_tokio::connection::new_session_sync;
use std::sync::Arc;
use tile::Tile;
use tokio;
use tokio::sync::mpsc::channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We can't do much until we have a D-Bus connection so just do it synchronously
    let (resource, conn) = new_session_sync()?;

    // Now start listening on our D-Bus connection
    tokio::spawn(async {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });

    let (sender, receiver) = channel(1024);

    let tiles: Vec<Arc<dyn Tile>> = vec![Arc::new(tiles::Time::new(0, sender.clone()))];

    for tile in tiles {
        tile.spawn();
    }

    loop {}
}
