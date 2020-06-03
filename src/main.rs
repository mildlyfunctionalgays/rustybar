pub mod output;
pub mod tile;
pub mod tiles;

use dbus_tokio::connection::new_session_sync;
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

    let mut index = 0usize;
    let mut wrap = |module| {
        let tile = tile::Tile::new(index, sender.clone(), Uuid::new_v4().to_string().into(), module);
        index += 1;
        tile
    };
    let tiles = vec![
        wrap(Box::new(tiles::Load::new())),
        wrap(Box::new(tiles::Memory::new())),
        wrap(Box::new(tiles::Hostname::new())),
        wrap(Box::new(tiles::Time::new())),
    ];

    let num_tiles = tiles.len();
    for tile in tiles.into_iter() {
        tile.spawn();
    }

    match output::launch(num_tiles, receiver).await? {}
}
