use serde::{ser::Serializer, Serialize};
use smart_default::SmartDefault;
use std::fmt::Debug;
use std::sync::Arc;
#[cfg(feature = "check_latency")]
use std::time::Instant;
//use tokio::sync::mpsc::{error::SendError, Sender};

#[allow(unused)]
#[derive(Copy, Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Alignment {
    Left,
    Center,
    Right,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::Center
    }
}

#[allow(unused)]
#[derive(Copy, Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Markup {
    None,
    Pango,
}

impl Default for Markup {
    fn default() -> Self {
        Self::None
    }
}

fn arc_default<S>(arc: &Arc<str>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(arc)
}

#[cfg(feature = "check_latency")]
fn time_diff<S>(&instant: &Instant, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let duration = Instant::now() - instant;
    //let duration = duration.as_secs_f32();
    serializer.serialize_str(&format!("{:?}", duration))
}

#[derive(Clone, Serialize, Debug, SmartDefault)]
pub struct Block {
    pub full_text: Box<str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_text: Option<Box<str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Box<str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "background")]
    pub background_color: Option<Box<str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "border")]
    pub border_color: Option<Box<str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_top: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_right: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_bottom: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_left: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<Alignment>,
    pub name: Box<str>,
    #[serde(serialize_with = "arc_default")]
    #[default = ""]
    pub instance: Arc<str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator_block_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markup: Option<Markup>,
    #[serde(serialize_with = "time_diff")]
    #[default(Instant::now())]
    #[cfg(feature = "check_latency")]
    pub latency: Instant,
}

#[derive(Clone, Debug)]
pub struct TileData {
    pub sender_id: usize,
    pub block: Block,
}
