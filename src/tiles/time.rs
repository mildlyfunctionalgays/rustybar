use crate::config::TimeConfig;
use crate::tile::Block;
use chrono::prelude::*;
use chrono::DateTime;
use futures::future::Future;
use futures::stream::Stream;
use futures_util::ready;
use pin_project::pin_project;
use std::error::Error;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::{delay_for, delay_until, Delay, Instant};

pub fn time_stream(
    config: TimeConfig,
) -> impl Stream<Item = Result<Block, Box<dyn std::error::Error + Send + Sync>>> {
    TimeStream {
        config,
        delay: delay_until(Instant::now()),
    }
}

#[pin_project]
struct TimeStream {
    config: TimeConfig,
    #[pin]
    delay: Delay,
}

impl TimeStream {
    fn send_time(&self, time: DateTime<Local>) -> Block {
        Block {
            full_text: time.format(&self.config.format).to_string().into(),
            short_text: Some(time.format(&self.config.short_format).to_string().into()),
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
    type Item = Result<Block, Box<dyn Error + Send + Sync>>;
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
