use crate::tile::{Block, Tile, TileData};
use std::io;
use std::str;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs::File;
use tokio::prelude::*;
use tokio::sync::mpsc::{error::SendError, Sender};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio::time::interval;

#[derive(Debug)]
pub struct Memory {
    sender_id: usize,
    sender: RwLock<Sender<TileData>>,
    instance: Arc<str>,
}

impl Memory {
    pub fn new(sender_id: usize, sender: Sender<TileData>, instance: Arc<str>) -> Memory {
        Memory {
            sender_id,
            sender: RwLock::new(sender),
            instance,
        }
    }

    async fn send(&self, data: TileData) -> Result<(), SendError<TileData>> {
        let mut sender = self.sender.write().await;
        sender.send(data).await
    }

    fn prettify_kib(kib: u64) -> Box<str> {
        if kib > u64::MAX / 1024 {
            panic!("Too much memory");
        }
        let mut mem = kib;
        let mut stages = 0u8;
        while mem >= 1024 {
            stages += 1;
            mem /= 1024;
        }
        format!(
            "{} {}iB",
            mem,
            match stages {
                0 => 'k',
                1 => 'M',
                2 => 'G',
                3 => 'T',
                4 => 'P',
                5 => 'E',
                6 => 'Z',
                _ => panic!("Too much memory, for real this time"),
            }
        )
        .into_boxed_str()
    }

    fn extract_value(line: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let mut parts = line.split_whitespace();
        parts.next();
        Ok(parts
            .next()
            .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?
            .parse()?)
    }

    async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut timer = interval(Duration::from_secs(5));
        let mut raw = [0u8; 256];
        loop {
            timer.tick().await;
            File::open("/proc/meminfo")
                .await?
                .read_exact(&mut raw)
                .await?;
            let string_data = str::from_utf8(&raw)?;
            let mut lines = string_data.split('\n');
            let mem_total = Memory::prettify_kib(Memory::extract_value(
                lines
                    .next()
                    .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?,
            )?);
            lines.next();
            let mem_avail = Memory::prettify_kib(Memory::extract_value(
                lines
                    .next()
                    .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?,
            )?);

            let full_text = format!("{} avail / {}", mem_avail, mem_total).into_boxed_str();
            let short_text = format!("{} / {}", mem_avail, mem_total).into_boxed_str();

            let block = Block {
                full_text,
                short_text: Some(short_text),
                instance: self.instance.clone(),
                name: "memory".into(),
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

impl Tile for Memory {
    fn spawn(self: Arc<Self>) -> JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>> {
        tokio::spawn(async move {
            let instance = self;
            let result = instance.run().await;
            eprintln!("Error in Memory: {:?}", result);
            result
        })
    }
}
