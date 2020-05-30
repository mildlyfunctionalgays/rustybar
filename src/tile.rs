use serde::Serialize;
use std::sync::Arc;
use tokio::task::JoinHandle;

#[derive(Copy, Clone, Serialize)]
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

#[derive(Copy, Clone, Serialize)]
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

#[derive(Clone, Serialize, Default)]
pub struct Block {
    full_text: Box<str>,
    short_text: Option<Box<str>>,
    color: Option<Box<str>>,
    #[serde(rename = "background")]
    background_color: Option<Box<str>>,
    #[serde(rename = "border")]
    border_color: Option<Box<str>>,
    border_top: Option<u32>,
    border_right: Option<u32>,
    border_bottom: Option<u32>,
    border_left: Option<u32>,
    min_width: Option<u32>,
    align: Option<Alignment>,
    name: Box<str>,
    instance: Box<str>,
    urgent: Option<bool>,
    separator: Option<bool>,
    separator_block_width: Option<u32>,
    markup: Option<Markup>,
}

#[derive(Clone)]
pub struct TileData {
    sender_id: usize,
    block: Block,
}

pub trait Tile: Send {
    fn spawn(self: Arc<Self>) -> JoinHandle<()>;
}
