use crate::tile::Block;
use crate::tiles;
use futures::stream::BoxStream;
use serde::Deserialize;
use smart_default::SmartDefault;
use std::env::var;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;
use tokio::fs::File;
use tokio::prelude::*;
use tokio::time;

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

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TileConfig {
    Memory,
    Load,
    Hostname,
    Time(TimeConfig),
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
    Ok(toml::from_slice(&config_contents)?)
}

pub fn process_tile(
    tile: &TileConfig,
) -> BoxStream<'static, Result<Block, Box<dyn std::error::Error + Send + Sync>>> {
    let five_secs = time::Duration::from_secs(5);
    match tile {
        TileConfig::Load => Box::pin(tiles::load_stream(time::interval(five_secs))),
        TileConfig::Memory => Box::pin(tiles::memory_stream(time::interval(five_secs))),
        TileConfig::Hostname => Box::pin(tiles::hostname_stream()),
        TileConfig::Time(c) => Box::pin(tiles::time_stream(c.clone())),
    }
}
