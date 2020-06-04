use crate::config::TimeConfig;
use crate::tile::{Block, BlockSender, TileModule};
use async_trait::async_trait;
use chrono::prelude::*;
use chrono::DateTime;
use futures::future::Future;
use futures::stream::Stream;
use futures_util::ready;
use pin_project::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time::{delay_for, delay_until, Delay, Instant};

#[derive(Debug)]
pub struct Time {
    format: Box<str>,
    short_format: Box<str>,
}

impl Time {
    pub fn new() -> Time {
        Default::default()
    }

    pub fn from_config(config: &TimeConfig) -> Time {
        Time {
            format: config.format.clone(),
            short_format: config.short_format.clone(),
        }
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
    async fn run(
        &mut self,
        sender: &mut BlockSender,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

pub fn time_stream(
    format: Box<str>,
    short_format: Box<str>,
) -> impl Stream<Item = Result<Block, Box<dyn std::error::Error + Send + Sync>>> {
    TimeStream {
        format,
        short_format,
        delay: delay_until(Instant::now()),
    }
}

#[pin_project]
struct TimeStream {
    format: Box<str>,
    short_format: Box<str>,
    #[pin]
    delay: Delay,
}

impl TimeStream {
    fn send_time(&self, time: DateTime<Local>) -> Block {
        Block {
            full_text: time.format(&self.format).to_string().into(),
            short_text: Some(time.format(&self.short_format).to_string().into()),
            name: "time".into(),
            ..Default::default()
        }
    }

    fn wait_for_next_second(now: DateTime<Local>) -> Delay {
        let next = now.trunc_subsecs(0) + chrono::Duration::seconds(1);
        let difference = next - now;

        delay_for(difference.to_std().unwrap())
    }
}

impl Stream for TimeStream {
    type Item = Result<Block, Box<dyn std::error::Error + Send + Sync>>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let project = Pin::as_mut(&mut self).project();
        ready!(Future::poll(project.delay, cx));

        let now = Local::now();
        Pin::as_mut(&mut self)
            .project()
            .delay
            .set(TimeStream::wait_for_next_second(now));
        Poll::Ready(Some(Ok(self.send_time(now))))
    }
}
