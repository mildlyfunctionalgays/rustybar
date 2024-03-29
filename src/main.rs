#![feature(generators)]
#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]
#![feature(never_type)]

mod async_iter;
mod config;
mod output;
mod tile;
mod tiles;

use dbus_tokio::connection::new_system_sync;
use futures::channel::mpsc::{channel, Sender};
use futures::{stream::BoxStream, StreamExt};
use std::fmt::Debug;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<!, Box<dyn std::error::Error>> {
    let config = config::read_config().await?;

    // We can't do much until we have a D-Bus connection so just do it synchronously
    let (resource, conn) = new_system_sync()?;

    // Now start listening on our D-Bus connection
    tokio::spawn(async {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });

    let (sender, receiver) = channel(1024);

    let tiles: Vec<_> = config
        .tile
        .iter()
        .map(|tile| config::process_tile(tile, &conn))
        .enumerate()
        .map(|(index, stream)| spawn_stream(index, stream, sender.clone()))
        .collect();

    let num_tiles = tiles.len();

    drop(sender);

    output::launch(num_tiles, receiver, config.default).await?
}

fn spawn_stream<E: 'static>(
    index: usize,
    stream: BoxStream<'static, Result<tile::Block, E>>,
    sender: Sender<Result<tile::TileData, E>>,
) where
    E: Debug + Send,
{
    tokio::spawn(async move {
        let instance: Arc<str> = Uuid::new_v4().to_string().into();
        let stream = stream.map(|block: Result<_, _>| {
            Ok(block.map(|block| tile::TileData {
                block: tile::Block {
                    instance: instance.clone(),
                    ..block
                },
                sender_id: index,
            }))
        });
        let future = stream.forward(sender);
        future.await
    });
}
