use serde::Serialize;
use std::sync::Arc;
use tokio::task::JoinHandle;

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

#[derive(Clone, Serialize, Default, Debug)]
pub struct Block {
    pub full_text: Box<str>,
    pub short_text: Option<Box<str>>,
    pub color: Option<Box<str>>,
    #[serde(rename = "background")]
    pub background_color: Option<Box<str>>,
    #[serde(rename = "border")]
    pub border_color: Option<Box<str>>,
    pub border_top: Option<u32>,
    pub border_right: Option<u32>,
    pub border_bottom: Option<u32>,
    pub border_left: Option<u32>,
    pub min_width: Option<u32>,
    pub align: Option<Alignment>,
    pub name: Box<str>,
    pub instance: Box<str>,
    pub urgent: Option<bool>,
    pub separator: Option<bool>,
    pub separator_block_width: Option<u32>,
    pub markup: Option<Markup>,
}

#[derive(Clone, Debug)]
pub struct TileData {
    pub sender_id: usize,
    pub block: Block,
}

pub trait Tile: Send {
    fn spawn(self: Arc<Self>) -> JoinHandle<()>;
}
