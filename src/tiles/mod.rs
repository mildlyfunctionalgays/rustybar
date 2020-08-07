pub mod battery;
pub mod hostname;
pub mod load;
pub mod memory;
pub mod time;
pub use battery::battery_stream;
pub use hostname::hostname_stream;
pub use load::load_stream;
pub use memory::memory_stream;
pub use time::time_stream;

use crate::tile::Block;
use std::error::Error;

pub type TileResult = Result<Block, Box<dyn Error + Send + Sync>>;
