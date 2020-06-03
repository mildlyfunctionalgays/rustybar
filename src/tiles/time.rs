use crate::tile::{Block, Tile, TileData};
use chrono::prelude::*;
use chrono::DateTime;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{error::SendError, Sender};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio::time::delay_for;

pub struct Time {
    sender_id: usize,
    sender: RwLock<Sender<TileData>>,
    instance: Arc<str>,
    format: Box<str>,
    short_format: Box<str>,
}

impl Time {
    pub fn new(sender_id: usize, sender: Sender<TileData>, instance: Arc<str>) -> Time {
        Time {
            sender_id,
            sender: RwLock::new(sender),
            instance,
            format: "%Y-%m-%d %H:%M:%S".into(),
            short_format: "%H:%M:%S".into(),
        }
    }

    async fn send(&self, data: TileData) -> Result<(), SendError<TileData>> {
        let mut sender = self.sender.write().await;
        sender.send(data).await
    }

    async fn send_time(&self, time: DateTime<Local>) -> Result<(), SendError<TileData>> {
        let block = Block {
            full_text: time.format(&self.format).to_string().into(),
            short_text: Some(time.format(&self.short_format).to_string().into()),
            instance: self.instance.clone(),
            name: "time".into(),
            ..Default::default()
        };
        let data = TileData {
            sender_id: self.sender_id,
            block,
        };
        self.send(data).await
    }

    async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut time = Local::now();
        loop {
            self.send_time(time).await?;
            time = Local::now();
            let millis_part = time.naive_local().timestamp_subsec_millis() as u64;
            let delay_ms = 1000u64 - millis_part % 1000; // Don't crash if we hit a leap second
            delay_for(Duration::from_millis(delay_ms)).await;
        }
    }
}

impl Tile for Time {
    fn spawn(self: Arc<Self>) -> JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>> {
        tokio::spawn(async move {
            let instance = self;
            instance.run().await
        })
    }
}
