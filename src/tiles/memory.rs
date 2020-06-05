use crate::tile::Block;
use futures::{Stream, StreamExt};
use std::io;
use std::str;
use tokio::fs::File;
use tokio::prelude::*;

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

pub fn memory_stream<T>(
    clock: T,
) -> impl Stream<Item = Result<Block, Box<dyn std::error::Error + Send + Sync>>>
where
    T: Stream,
{
    clock.then(|_| async {
        let mut raw = [0u8; 256];
        File::open("/proc/meminfo")
            .await?
            .read_exact(&mut raw)
            .await?;
        let string_data = str::from_utf8(&raw)?;
        let mut lines = string_data.split('\n');
        let mem_total = prettify_kib(extract_value(
            lines
                .next()
                .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?,
        )?);
        lines.next();
        let mem_avail = prettify_kib(extract_value(
            lines
                .next()
                .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?,
        )?);

        let full_text = format!("{} avail / {}", mem_avail, mem_total).into_boxed_str();
        let short_text = format!("{} / {}", mem_avail, mem_total).into_boxed_str();

        Ok(Block {
            full_text,
            short_text: Some(short_text),
            name: "memory".into(),
            ..Default::default()
        })
    })
}
