use crate::tile::Block;
use futures::{future::try_join3, stream, Stream};
use futures_util::StreamExt;
use std::error::Error;
use tokio::fs::File;
use tokio::prelude::*;

pub fn battery_stream() -> impl Stream<Item = Result<Block, Box<dyn Error + Send + Sync>>> {
    stream::repeat(()).then(|()| async {
        let charge_now = async {
            let mut raw = String::new();
            File::open("/sys/class/power_supply/BAT0/charge_now")
                .await?
                .read_to_string(&mut raw)
                .await?;
            let charge: u32 = raw.trim_end().parse()?;
            Result::<_, Box<dyn Error + Send + Sync>>::Ok(charge)
        };
        let charge_total = async {
            let mut raw = String::new();
            File::open("/sys/class/power_supply/BAT0/charge_full")
                .await?
                .read_to_string(&mut raw)
                .await?;
            let charge: u32 = raw.trim_end().parse()?;
            Result::<_, Box<dyn Error + Send + Sync>>::Ok(charge)
        };
        let status = async {
            let mut raw = String::new();
            File::open("/sys/class/power_supply/BAT0/status")
                .await?
                .read_to_string(&mut raw)
                .await?;
            raw.truncate(raw.trim_end().len());
            Result::<_, Box<dyn Error + Send + Sync>>::Ok(raw)
        };
        let (charge_now, charge_total, status) =
            try_join3(charge_now, charge_total, status).await?;
        let percentage = charge_now * 100 / charge_total;
        Ok(Block {
            full_text: format!("{}% {}", percentage, status).into(),
            short_text: format!("{}%", percentage).into_boxed_str().into(),
            name: "battery".into(),
            ..Default::default()
        })
    })
}
