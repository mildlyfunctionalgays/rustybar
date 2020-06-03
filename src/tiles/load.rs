use crate::tile::{Block, BlockSender, TileModule};
use async_trait::async_trait;
use std::time::Duration;
use tokio::fs::File;
use tokio::prelude::*;
use tokio::time::interval;

#[derive(Debug, Default)]
pub struct Load;

impl Load {
    pub fn new() -> Self {
        Load
    }
}

#[async_trait]
impl TileModule for Load {
    async fn run(
        &mut self,
        sender: &mut BlockSender,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
                name: "load".into(),
                ..Default::default()
            };
            sender.send(block).await?;
        }
    }
}
