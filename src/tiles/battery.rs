use crate::config::BatteryConfig;
use crate::tile::Block;
use futures::future::try_join3;
use futures_async_stream::try_stream;
use std::error::Error;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[try_stream(ok = Block, error = Box<dyn Error + Send + Sync>)]
pub async fn battery_stream(config: BatteryConfig) {
    let base_path = Path::new("/sys/class/power_supply").join(&*config.battery);
    loop {
        let charge_now = async {
            let mut raw = String::new();
            File::open(base_path.join("energy_now"))
                .await?
                .read_to_string(&mut raw)
                .await?;
            let charge: u32 = raw.trim_end().parse()?;
            Result::<_, Box<dyn Error + Send + Sync>>::Ok(charge)
        };
        let charge_total = async {
            let mut raw = String::new();
            File::open(base_path.join("energy_full"))
                .await?
                .read_to_string(&mut raw)
                .await?;
            let charge: u32 = raw.trim_end().parse()?;
            Result::<_, Box<dyn Error + Send + Sync>>::Ok(charge)
        };
        let status = async {
            let mut raw = String::new();
            File::open(base_path.join("status"))
                .await?
                .read_to_string(&mut raw)
                .await?;
            raw.truncate(raw.trim_end().len());
            Result::<_, Box<dyn Error + Send + Sync>>::Ok(raw)
        };
        let (charge_now, charge_total, status) =
            try_join3(charge_now, charge_total, status).await?;
        let percentage = charge_now * 100 / charge_total;
        yield Block {
            full_text: format!("{}% {}", percentage, status).into(),
            short_text: format!("{}%", percentage).into_boxed_str().into(),
            name: "battery".into(),
            ..Default::default()
        };
    }
}
