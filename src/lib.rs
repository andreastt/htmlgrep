extern crate kuchiki;

pub mod flag;
pub mod grep;
pub mod path;
pub mod result;

pub use grep::{select, Match, Matches};
pub use result::{Error, Result};
