use crate::tile::{Block, Tile, TileData};
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;
use tokio::time;
use uuid::Uuid;

pub struct Time {
    sender_id: usize,
    sender: Sender<TileData>,
    instance: Box<str>,
}

impl Time {
    pub fn new(sender_id: usize, sender: Sender<TileData>) -> Time {
        let instance = Uuid::new_v4().to_string().into_boxed_str();
        Time {
            sender_id,
            sender,
            instance,
        }
    }

    async fn run(&self) {}
}

impl Tile for Time {
    fn spawn(self: Arc<Self>) -> JoinHandle<()> {
        tokio::spawn(async {
            let instance = self;
            instance.run().await
        })
    }
}
