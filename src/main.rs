pub mod output;
pub mod tile;
pub mod tiles;

use dbus_tokio::connection::new_session_sync;
use std::sync::Arc;
use tile::Tile;
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

    let tiles: Vec<Arc<dyn Tile>> = vec![
        Arc::new(tiles::Load::new(
            0,
            sender.clone(),
            Uuid::new_v4().to_string().into_boxed_str(),
        )),
        Arc::new(tiles::Hostname::new(
            1,
            sender.clone(),
            Uuid::new_v4().to_string().into_boxed_str(),
        )),
        Arc::new(tiles::Time::new(
            2,
            sender,
            Uuid::new_v4().to_string().into_boxed_str(),
        )),
    ];

    for tile in &tiles {
        tile.clone().spawn();
    }

    let num_tiles = tiles.len();
    output::launch(num_tiles, receiver).await?;

    Ok(())
}
