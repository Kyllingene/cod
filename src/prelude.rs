//! Imports everything you need in an ergonomic fashion.
//!
//! Imports the following:
//! - [`clear`]
//! - [`color`]
//! - [`goto`]
//! - [`guard`]
//! - [`rect`]
//! - [`style`]
//! - [`read`] (with feature `input` enabled)

pub use crate::{clear, color, goto, guard, rect, style};

#[cfg(feature = "crossterm")]
pub use crate::{read, term};
