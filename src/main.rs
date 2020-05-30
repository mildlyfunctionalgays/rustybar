pub mod output;
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
    let (resource, _conn) = new_session_sync()?;

    // Now start listening on our D-Bus connection
    tokio::spawn(async {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });

    let (sender, receiver) = channel(1024);

    let instance = Uuid::new_v4().to_string().into_boxed_str();
    let tiles: Vec<Arc<dyn Tile>> = vec![Arc::new(tiles::Time::new(0, sender, instance))];

    for tile in &tiles {
        tile.clone().spawn();
    }

    let num_tiles = tiles.len();
    tokio::spawn(async move {
        output::launch(num_tiles, receiver).await.unwrap();
    });

    loop {}
}
