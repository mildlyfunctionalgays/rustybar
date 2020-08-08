use crate::tile::Block;
use futures_async_stream::try_stream;
use std::error::Error;
use tokio::fs::File;
use tokio::prelude::*;

#[try_stream(ok = Block, error = Box<dyn Error + Send + Sync>)]
pub async fn load_stream() {
    loop {
        let mut raw = String::new();
        let mut file = File::open("/proc/loadavg").await?;
        file.read_to_string(&mut raw).await?;
        let (load, _rest) = raw.split_at(raw.find(' ').unwrap_or(0));
        yield Block {
            full_text: load.into(),
            name: "load".into(),
            ..Default::default()
        };
    }
}
