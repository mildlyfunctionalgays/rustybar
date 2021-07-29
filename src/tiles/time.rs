use super::TileResult;
use crate::config::TimeConfig;
use crate::tile::Block;
use chrono::prelude::*;
use futures_async_stream::stream;
use tokio::time::sleep;

#[stream(item = TileResult)]
pub async fn time_stream(config: TimeConfig) {
    loop {
        let now = Local::now();
        yield Ok(Block {
            full_text: now.format(&config.format).to_string().into(),
            short_text: Some(now.format(&config.short_format).to_string().into()),
            name: "time".into(),
            ..Default::default()
        });

        let next = now.trunc_subsecs(0) + chrono::Duration::seconds(1);
        let difference = next - now;

        sleep(difference.to_std().unwrap()).await;
    }
}
