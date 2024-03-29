use crate::tiles;
use crate::tiles::TileResult;
use dbus::nonblock::SyncConnection;
use futures::{stream::BoxStream, Stream};
use serde::{Deserialize, Deserializer};
use smart_default::SmartDefault;
use std::env::var;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use structopt::StructOpt;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::time::Duration;
use tokio_stream::StreamExt;

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Config {
    pub default: DefaultSection,
    pub tile: Box<[TileConfig]>,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct DefaultSection {
    pub spacing: Option<u32>,
}

fn deserialize_duration<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let number = s.trim_end_matches(char::is_alphabetic);
    let suffix = s.trim_start_matches(number);
    let number: f64 = number.parse().expect("Not a valid f64");
    let duration = match suffix {
        "s" | "" => Duration::from_secs_f64(number),
        "m" => Duration::from_secs_f64(number * 60f64),
        "ms" => Duration::from_secs_f64(number / 1000f64),
        _ => unimplemented!(),
    };
    Ok(Some(duration))
}

#[derive(Deserialize, Clone, Debug)]
pub struct TileConfig {
    #[serde(flatten)]
    config_type: TileConfigType,
    #[serde(deserialize_with = "deserialize_duration", default)]
    update: Option<Duration>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TileConfigType {
    Battery(BatteryConfig),
    Memory,
    Load,
    Hostname,
    Time(TimeConfig),
    Iwd(IwdConfig),
}

#[derive(SmartDefault, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct BatteryConfig {
    #[default("BAT0")]
    pub battery: Box<str>,
}

#[derive(SmartDefault, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct IwdConfig {
    #[default("wlan0")]
    pub interface: Box<str>,
}

#[derive(SmartDefault, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct TimeConfig {
    #[default("%Y-%m-%d %H:%M:%S")]
    pub format: Box<str>,
    #[default("%H:%M:%S")]
    pub short_format: Box<str>,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rustybar",
    about = "Something to exec for your swaybar or i3bar"
)]
struct Args {
    /// Configuration file, default is $XDG_CONFIG_HOME/rustybar/config.toml
    #[structopt(short, long, parse(from_os_str))]
    pub config: Option<PathBuf>,
}

pub async fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let args = Args::from_args();
    let config_path = match args.config {
        Some(config) => config,
        None => {
            if let Ok(rustybar_config_env) = var("RUSTYBAR_CONFIG") {
                rustybar_config_env.into()
            } else if let Ok(xdg_config_home) = var("XDG_CONFIG_HOME") {
                [&xdg_config_home, "rustybar", "config.toml"]
                    .iter()
                    .collect()
            } else if let Ok(home) = var("HOME") {
                [&home, ".config", "rustybar", "config.toml"]
                    .iter()
                    .collect()
            } else {
                return Err(Box::new(io::Error::new(
                            io::ErrorKind::NotFound,
                            "Could not find RUSTYBAR_CONFIG, XDG_CONFIG_HOME, or HOME environment variables"
                        )));
            }
        }
    };

    let mut config_contents = vec![];
    File::open(config_path)
        .await?
        .read_to_end(&mut config_contents)
        .await?;
    Ok(toml::from_str(std::str::from_utf8(&config_contents)?)?)
}

pub fn process_tile(
    tile: &TileConfig,
    connection: &Arc<SyncConnection>,
) -> BoxStream<'static, TileResult> {
    let five_secs = Duration::from_secs(5);
    match &tile.config_type {
        TileConfigType::Battery(c) => wrap(
            tiles::battery_stream(c.clone()),
            tile.update.or(Some(five_secs)),
        ),
        TileConfigType::Hostname => wrap(tiles::hostname_stream(connection.as_ref()), tile.update),
        TileConfigType::Load => wrap(tiles::load_stream(), tile.update.or(Some(five_secs))),
        TileConfigType::Memory => wrap(tiles::memory_stream(), tile.update.or(Some(five_secs))),
        TileConfigType::Time(c) => wrap(tiles::time_stream(c.clone()), tile.update),
        TileConfigType::Iwd(c) => wrap(
            tiles::iwd_stream(connection.clone(), c.clone()),
            tile.update.or(Some(five_secs)),
        ),
    }
}

fn wrap<'a, S>(stream: S, duration: Option<Duration>) -> BoxStream<'a, TileResult>
where
    S: Stream<Item = TileResult> + Send + 'a,
{
    match duration {
        Some(duration) => Box::pin(stream.throttle(duration)),
        None => Box::pin(stream),
    }
}
