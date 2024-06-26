//! A small library for command-line drawing.
#![warn(clippy::pedantic)]
#![warn(missing_docs)]

use std::io::{stdout, Write};

#[cfg(feature = "crossterm")]
pub use crossterm;

pub mod clear;
pub mod color;
pub mod goto;
pub mod guard;
pub mod prelude;
pub mod rect;
pub mod style;
pub mod term;

mod line;
mod println;

#[cfg(feature = "crossterm")]
pub mod read;

/// The user attempted to draw a non-orthogonal line through an orthogonal
/// function, such as [`orth_line`] or [`rect::line`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NonOrthogonal;

/// Print an escape sequence.
fn escape<T: std::fmt::Display>(code: T) {
    print!("{}[{}", 27 as char, code);
}

/// Disable all style and color attributes.
pub fn normal() {
    escape("0m");
}

/// Draw a single character onto the screen.
pub fn pixel(c: char, x: u32, y: u32) {
    escape(format!("{};{}H{}", y + 1, x + 1, c));
}

/// Draw an orthogonal line to the screen.
///
/// # Errors
///
/// If the given line is non-orthogonal, returns an error.
pub fn orth_line(c: char, x1: u32, y1: u32, x2: u32, y2: u32) -> Result<(), NonOrthogonal> {
    if x1 != x2 && y1 != y2 {
        return Err(NonOrthogonal);
    }

    if x1 == x2 {
        let mut y = y1.min(y2);

        while y != y1.max(y2) + 1 {
            pixel(c, x1, y);

            y += 1;
        }
    } else {
        let mut x = x1.min(x2);

        while x != x1.max(x2) + 1 {
            pixel(c, x, y1);

            x += 1;
        }
    }

    Ok(())
}

/// Draw a line onto the screen.
#[allow(clippy::missing_panics_doc)]
pub fn line(c: char, x1: u32, y1: u32, x2: u32, y2: u32) {
    if x1 == x2 || y1 == y2 {
        orth_line(c, x1, x2, y1, y2).unwrap();
        return;
    }

    for (x, y) in line::Iter::new(x1, y1, x2, y2) {
        pixel(c, x, y);
    }
}

/// Draw a "texture" onto the screen.
pub fn blit<S: AsRef<str>>(src: S, mut x: u32, mut y: u32) {
    let src = src.as_ref();
    let rows = src.split('\n').map(|s| s.chars());

    let ox = x;
    for row in rows {
        for c in row {
            pixel(c, x, y);
            x += 1;
        }
        x = ox;
        y += 1;
    }
}

/// Draw a "texture" onto the screen, skipping over spaces.
/// Replaces all `blank`s with actual spaces.
///
/// Example:
///
/// ```rust
/// # use cod::prelude::*;
///
/// // prints `foobar`
/// cod::blit("foobar", 0, 0);
/// // updates to `to ban`
/// cod::blit_transparent("t _  n", '_', 0, 0);
/// ```
pub fn blit_transparent<S: AsRef<str>>(src: S, blank: char, mut x: u32, mut y: u32) {
    let src = src.as_ref();
    let rows = src.split('\n').map(|s| s.chars());

    let ox = x;
    for row in rows {
        for c in row {
            match c {
                ' ' => goto::right(1),
                ch if ch == blank => pixel(' ', x, y),
                _ => pixel(c, x, y),
            }
            x += 1;
        }
        x = ox;
        y += 1;
    }
}

/// Draw a triangle onto the screen.
pub fn triangle(c: char, x1: u32, y1: u32, x2: u32, y2: u32, x3: u32, y3: u32) {
    line(c, x1, y1, x2, y2);
    line(c, x2, y2, x3, y3);
    line(c, x1, y1, x3, y3);
}

// TODO: do this ever
// /// Draw a filled triangle onto the screen.
// pub fn triangle_fill(c: char, x1: u32, y1: u32, x2: u32, y2: u32, x3: u32, y3: u32) {
//     todo!()
// }

/// Draw text onto the screen (non-wrapping, but respects linebreaks).
pub fn text<S: AsRef<str>>(s: S, x: u32, mut y: u32) {
    let chars = s.as_ref().chars();
    let mut nx = x;
    for ch in chars {
        if ch == '\n' {
            nx = x;
            y += 1;
        }

        pixel(ch, nx, y);
        nx += 1;
    }
}

/// Flush to stdout.
///
/// # Panics
///
/// If flushing fails, panics with `Failed to flush to stdout`.
pub fn flush() {
    stdout().flush().expect("Failed to flush stdout");
}
