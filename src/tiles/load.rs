use crate::tile::Block;
use futures::stream::StreamExt;
use tokio::fs::File;
use tokio::prelude::*;
use tokio::stream::Stream;

pub fn load_stream() -> impl Stream<Item = Result<Block, Box<dyn std::error::Error + Send + Sync>>>
{
    futures::stream::repeat(()).then(|()| async {
        let mut raw = String::new();
        File::open("/proc/loadavg")
            .await?
            .read_to_string(&mut raw)
            .await?;
        let (load, _rest) = raw.split_at(raw.find(' ').unwrap_or(0));
        Ok(Block {
            full_text: load.into(),
            name: "load".into(),
            ..Default::default()
        })
    })
}
