use crate::tile::{Block, Tile, TileData};
use std::sync::Arc;
use std::time::Duration;
use tokio::fs::File;
use tokio::prelude::*;
use tokio::sync::mpsc::{error::SendError, Sender};
use tokio::task::JoinHandle;
use tokio::time::interval;

#[derive(Debug)]
pub struct Load {
    sender_id: usize,
    sender: Sender<TileData>,
    instance: Arc<str>,
}

impl Load {
    pub fn new(sender_id: usize, sender: Sender<TileData>, instance: Arc<str>) -> Load {
        Load {
            sender_id,
            sender,
            instance,
        }
    }

    async fn send(&mut self, data: TileData) -> Result<(), SendError<TileData>> {
        self.sender.send(data).await
    }

    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
    fn spawn(mut self: Box<Self>) -> JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>> {
        tokio::spawn(async move {
            self.run().await
        })
    }
}
