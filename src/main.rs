pub mod config;
pub mod output;
pub mod tile;
pub mod tiles;

use dbus_tokio::connection::new_session_sync;
use futures::channel::mpsc::{channel, Sender};
use futures::{Stream, StreamExt};
use std::fmt::Debug;
use std::sync::Arc;
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
    // let format = "%Y-%m-%d %H:%M:%S".into();
    // let short_format = "%H:%M:%S".into();
    // let stream = tiles::time::time_stream(format, short_format);
    // spawn_stream(4, stream, sender.clone());

    drop(sender);

    match output::launch(num_tiles, receiver, config.default).await? {}
}

#[allow(unused)]
fn spawn_stream<S, E>(index: usize, stream: S, sender: Sender<tile::TileData>)
where
    S: Stream<Item = Result<tile::Block, E>> + Send + 'static,
    E: Debug,
{
    tokio::spawn(async move {
        let instance: Arc<str> = Uuid::new_v4().to_string().into();
        let stream = stream.map(|block| {
            Ok(tile::TileData {
                block: tile::Block {
                    instance: instance.clone(),
                    ..block.unwrap()
                },
                sender_id: index,
            })
        });
        let future = stream.forward(sender);
        future.await
    });
}
