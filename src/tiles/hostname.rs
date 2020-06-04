use crate::tile::{Block, BlockSender, TileModule};
use async_trait::async_trait;
use futures::stream;
use futures::Stream;
use tokio::fs::File;
use tokio::prelude::*;

#[derive(Debug, Default)]
pub struct Hostname;

impl Hostname {
    pub fn new() -> Hostname {
        Hostname
    }
}

#[async_trait]
impl TileModule for Hostname {
    async fn run(
        &mut self,
        sender: &mut BlockSender,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut raw = String::new();
        File::open("/proc/sys/kernel/hostname")
            .await?
            .read_to_string(&mut raw)
            .await?;
        let block = Block {
            full_text: raw.trim_end_matches('\n').into(),
            name: "hostname".into(),
            ..Default::default()
        };
        sender.send(block).await?;
        // What's the hostname gonna do? Change?
        Ok(())
    }
}

#[allow(unused)]
fn hostname_stream() -> impl Stream<Item = Result<Block, Box<dyn std::error::Error + Send + Sync>>>
{
    stream::once(async {
        let mut raw = String::new();
        File::open("/proc/sys/kernel/hostname")
            .await?
            .read_to_string(&mut raw)
            .await?;
        let block = Block {
            full_text: raw.trim_end_matches('\n').into(),
            name: "hostname".into(),
            ..Default::default()
        };
        Ok(block)
    })
}
