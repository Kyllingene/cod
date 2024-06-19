#![allow(missing_docs)]

pub use crate::{clear, color, goto, guard, rect, style, term};

#[cfg(feature = "crossterm")]
pub use crate::read;
