use crate::tile::Block;
use futures_async_stream::try_stream;
use std::error::Error;
use std::{io, str, u64};
use tokio::fs::File;
use tokio::prelude::*;

fn prettify_kib(kib: u64) -> Box<str> {
    let (mem, unit) = match kib {
        0..=0x3ff => (kib, 'k'),
        0x400..=0xfffff => (kib >> 10, 'M'),
        0x100000..=0x3fffffff => (kib >> 20, 'G'),
        0x40000000..=0xffffffffff => (kib >> 30, 'T'),
        0x10000000000..=0x3ffffffffffff => (kib >> 40, 'P'),
        0x4000000000000..=0xfffffffffffffff => (kib >> 50, 'E'),
        0x1000000000000000..=0xffffffffffffffff => (kib >> 60, 'Z'),
    };
    format!("{} {}iB", mem, unit).into_boxed_str()
}

fn extract_value(line: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let mut parts = line.split_whitespace();
    parts.next();
    Ok(parts
        .next()
        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?
        .parse()?)
}

#[try_stream(ok = Block, error = Box<dyn Error + Send + Sync>)]
pub async fn memory_stream() {
    loop {
        let mut raw = [0u8; 256];
        let mut file = File::open("/proc/meminfo").await?;
        file.read_exact(&mut raw).await?;
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

        yield Block {
            full_text,
            short_text: Some(short_text),
            name: "memory".into(),
            ..Default::default()
        };
    }
}
