use async_trait::async_trait;
use serde::{ser::Serializer, Serialize};
use smart_default::SmartDefault;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::mpsc::{error::SendError, Sender};
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

fn arc_default<S>(arc: &Arc<str>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(arc)
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
    #[default = r#""".into()"#]
    pub instance: Arc<str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator_block_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markup: Option<Markup>,
}

#[derive(Clone, Debug)]
pub struct TileData {
    pub sender_id: usize,
    pub block: Block,
}

#[async_trait]
pub trait TileModule: Send + std::fmt::Debug {
    async fn run(&mut self, sender: &mut BlockSender) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

#[derive(Debug)]
pub struct BlockSender {
    sender_id: usize,
    sender: Sender<TileData>,
    instance: Arc<str>,
}

impl BlockSender {
    pub async fn send(&mut self, mut block: Block) -> Result<(), SendError<TileData>> {
        block.instance = self.instance.clone();
        let data = TileData {
            block,
            sender_id: self.sender_id,
        };
        self.sender.send(data).await
    }
}

#[derive(Debug)]
pub struct Tile {
    sender: BlockSender,
    module: Box<dyn TileModule>,
}

impl Tile {
    pub fn new(sender_id: usize, sender: Sender<TileData>, instance: Arc<str>, module: Box<dyn TileModule>) -> Self {
        Tile {
            sender: BlockSender {
                sender_id,
                sender,
                instance,
            },
            module,
        }
    }
    pub fn spawn(mut self) -> JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>> {
        tokio::spawn(async move {
            self.module.run(&mut self.sender).await
        })
    }
}