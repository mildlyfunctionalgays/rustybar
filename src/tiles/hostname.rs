use crate::tile::Block;
use futures::stream;
use futures::Stream;
use tokio::fs::File;
use tokio::prelude::*;

pub fn hostname_stream(
) -> impl Stream<Item = Result<Block, Box<dyn std::error::Error + Send + Sync>>> {
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
