use crate::tile::{Block, Tile, TileData};
use std::sync::Arc;
use std::time::Duration;
use tokio::fs::File;
use tokio::prelude::*;
use tokio::sync::mpsc::{error::SendError, Sender};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio::time::interval;

pub struct Load {
    sender_id: usize,
    sender: RwLock<Sender<TileData>>,
    instance: Arc<str>,
}

impl Load {
    pub fn new(sender_id: usize, sender: Sender<TileData>, instance: Arc<str>) -> Load {
        Load {
            sender_id,
            sender: RwLock::new(sender),
            instance,
        }
    }

    async fn send(&self, data: TileData) -> Result<(), SendError<TileData>> {
        let mut sender = self.sender.write().await;
        sender.send(data).await
    }

    async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut timer = interval(Duration::from_secs(5));
        loop {
            timer.tick().await;
            let mut raw = String::new();
            File::open("/proc/loadavg")
                .await?
                .read_to_string(&mut raw)
                .await?;
            let (load, _rest) = raw.split_at(raw.find(' ').unwrap_or(0));
            let block = Block {
                full_text: load.into(),
                instance: self.instance.clone(),
                name: "load".into(),
                ..Default::default()
            };
            let data = TileData {
                block,
                sender_id: self.sender_id,
            };
            self.send(data).await?;
        }
    }
}

impl Tile for Load {
    fn spawn(self: Arc<Self>) -> JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>> {
        tokio::spawn(async move {
            let instance = self;
            instance.run().await
        })
    }
}
