pub mod config;
pub mod output;
pub mod tile;
pub mod tiles;

use dbus_tokio::connection::new_session_sync;
use tokio::sync::mpsc::channel;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::read_config().await?;

    // We can't do much until we have a D-Bus connection so just do it synchronously
    let (resource, _conn) = new_session_sync()?;

    // Now start listening on our D-Bus connection
    tokio::spawn(async {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });

    let (sender, receiver) = channel(1024);

    let mut index = 0usize;
    let wrap = |module| {
        let tile = tile::Tile::new(
            index,
            sender.clone(),
            Uuid::new_v4().to_string().into(),
            module,
        );
        index += 1;
        tile
    };

    let tiles: Vec<tile::Tile> = config
        .tile
        .iter()
        .map(config::process_tile)
        .map(wrap)
        .collect();

    let num_tiles = tiles.len();
    for tile in tiles.into_iter() {
        tile.spawn();
    }

    match output::launch(num_tiles, receiver, config.default).await? {}
}
