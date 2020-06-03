use crate::tile::{Block, BlockSender, TileModule};
use async_trait::async_trait;
use chrono::prelude::*;
use chrono::DateTime;
use std::time::Duration;
use tokio::time::delay_for;

#[derive(Debug)]
pub struct Time {
    format: Box<str>,
    short_format: Box<str>,
}

impl Time {
    pub fn new() -> Time {
        Default::default()
    }

    fn send_time(&mut self, time: DateTime<Local>) -> Block {
        Block {
            full_text: time.format(&self.format).to_string().into(),
            short_text: Some(time.format(&self.short_format).to_string().into()),
            name: "time".into(),
            ..Default::default()
        }
    }
}

impl Default for Time {
    fn default() -> Self {
        Time {
            format: "%Y-%m-%d %H:%M:%S".into(),
            short_format: "%H:%M:%S".into(),
        }
    }
}

#[async_trait]
impl TileModule for Time {
    async fn run(&mut self, sender: &mut BlockSender) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut time = Local::now();
        loop {
            sender.send(self.send_time(time)).await?;
            time = Local::now();
            let millis_part = time.naive_local().timestamp_subsec_millis() as u64;
            let delay_ms = 1000u64 - millis_part % 1000; // Don't crash if we hit a leap second
            delay_for(Duration::from_millis(delay_ms)).await;
        }
    }
}
