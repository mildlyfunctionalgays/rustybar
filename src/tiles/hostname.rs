use crate::tile::{Block, Tile, TileData};
use std::sync::Arc;
use tokio::fs::File;
use tokio::prelude::*;
use tokio::sync::mpsc::{error::SendError, Sender};
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct Hostname {
    sender_id: usize,
    sender: Sender<TileData>,
    instance: Arc<str>,
}

impl Hostname {
    pub fn new(sender_id: usize, sender: Sender<TileData>, instance: Arc<str>) -> Hostname {
        Hostname {
            sender_id,
            sender,
            instance,
        }
    }

    async fn send(&mut self, data: TileData) -> Result<(), SendError<TileData>> {
        self.sender.send(data).await
    }

    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut raw = String::new();
        File::open("/proc/sys/kernel/hostname")
            .await?
            .read_to_string(&mut raw)
            .await?;
        let block = Block {
            full_text: raw.trim_end_matches('\n').into(),
            instance: self.instance.clone(),
            name: "hostname".into(),
            ..Default::default()
        };
        let data = TileData {
            block,
            sender_id: self.sender_id,
        };
        self.send(data).await?;
        // What's the hostname gonna do? Change?
        Ok(())
    }
}

impl Tile for Hostname {
    fn spawn(mut self: Box<Self>) -> JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>> {
        tokio::spawn(async move {
            self.run().await
        })
    }
}
