mod amount;
mod depth;
mod depthmap;
mod depths;
mod liq;
mod order_placed;
mod order_to_place;
mod price;
mod side;
mod spread;
mod symbol;
mod trade;
mod worth;

pub mod depth_util;
pub mod my_date_formatter;
pub mod my_duration_formatter;

pub use amount::*;
pub use depth::*;
pub use depthmap::*;
pub use depths::*;
pub use liq::*;
pub use order_placed::*;
pub use order_to_place::*;
pub use price::*;
pub use side::*;
pub use spread::*;
pub use symbol::*;
pub use trade::*;
pub use worth::*;

use std::collections::HashMap;

pub type BotId = String;
pub type Currency = String;
pub type Exchange = String;

pub type LiqByCurrency = HashMap<Currency, Liq>;
pub type Balances = HashMap<Currency, Amount>;
