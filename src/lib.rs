//! A utility for command-line drawing.
//!
//! Many utilities are at the top-level; however, there are a couple of modules:
//!  - [`clear`]: Functions for clearing the screen or sections thereof.
//!  - [`color`]: Functions for setting the foreground/background color.
//!  - [`style`]: Functions for styling text, i.e. bold, italic, underline.
//!  - [`goto`]: Functions for moving the cursor around the screen.
//!  - [`read`]: Functions for reading from stdin. Must specify feature `input` to use.
//!
//! Both the `color` and `style` modules have three parts:
//! - The root, containing base functions
//! - A `de` module, containing functions to reset the attributes
//! - A `with` module, containing functions to run code with certain attributes
#![warn(clippy::pedantic)]
#![warn(missing_docs)]

use std::io::{stdout, Write};

#[cfg(feature = "input")]
pub use console::{self, Key};

pub mod clear;
pub mod color;
pub mod goto;
pub mod guard;
pub mod prelude;
pub mod style;

mod line;

#[cfg(feature = "input")]
pub mod read;

/// The user attempted to draw a non-orthogonal line through an orthogonal
/// function, such as [`orth_line`] or [`rect`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NonOrthogonal;

enum BoxDrawingChar {
    Horizontal,
    Vertical,

    TopLeftCorner,
    TopRightCorner,
    BottomLeftCorner,
    BottomRightCorner,

    TopT,
    LeftT,
    RightT,
    BottomT,
    MiddleT,
}

impl From<BoxDrawingChar> for char {
    fn from(val: BoxDrawingChar) -> char {
        match val {
            BoxDrawingChar::Horizontal => '\u{2550}',
            BoxDrawingChar::Vertical => '\u{2551}',

            BoxDrawingChar::TopLeftCorner => '\u{2554}',
            BoxDrawingChar::TopRightCorner => '\u{2557}',
            BoxDrawingChar::BottomLeftCorner => '\u{255A}',
            BoxDrawingChar::BottomRightCorner => '\u{255D}',

            BoxDrawingChar::TopT => '\u{2566}',
            BoxDrawingChar::LeftT => '\u{2560}',
            BoxDrawingChar::RightT => '\u{2563}',
            BoxDrawingChar::BottomT => '\u{2569}',
            BoxDrawingChar::MiddleT => '\u{256C}',
        }
    }
}

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

/// Draw a "texture" onto the screen, skipping spaces.
/// If you need to fill a blank, use a tab (`\t`) character.
pub fn blit_transparent<S: AsRef<str>>(src: S, mut x: u32, mut y: u32) {
    let src = src.as_ref();
    let rows = src.split('\n').map(|s| s.chars());

    let ox = x;
    for row in rows {
        for c in row {
            match c {
                ' ' => goto::right(1),
                '\t' => pixel(' ', x, y),
                _ => pixel(c, x, y),
            }
            x += 1;
        }
        x = ox;
        y += 1;
    }
}

/// Draw a rectangle onto the screen.
///
/// # Errors
///
/// If the given line is non-orthogonal, returns an error.
pub fn rect(c: char, x1: u32, y1: u32, x2: u32, y2: u32) -> Result<(), NonOrthogonal> {
    orth_line(c, x1, y1, x1, y2)?;
    orth_line(c, x1, y1, x2, y1)?;
    orth_line(c, x2, y2, x1, y2)?;
    orth_line(c, x2, y2, x2, y1)?;

    Ok(())
}

/// Print text using ASCII box-drawing characters.
///
/// Substitutions:
/// r - y - 7    ╔ ═ ╦ ═ ╗
/// |   |   |    ║   ║   ║
/// p - + - d    ╠ ═ ╬ ═ ╣
/// |   |   |    ║   ║   ║
/// l - t - j    ╚ ═ ╩ ═ ╝      
///
/// Putting a backslash before a character (e.g. "\\r" or "\\\\") will escape it.
pub fn ascii_box_chars<T: IntoIterator<Item = char>>(s: T, x: u32, mut y: u32) {
    let mut escaped = false;
    let mut nx = x;
    for c in s {
        if escaped {
            pixel(c, x, y);
            nx += 1;
            escaped = false;
            continue;
        }

        match c {
            '\\' => escaped = true,
            '\n' => {
                nx = x;
                y += 1;
                continue;
            }

            'r' => pixel(BoxDrawingChar::TopLeftCorner.into(), nx, y),
            '-' => pixel(BoxDrawingChar::Horizontal.into(), nx, y),
            'y' => pixel(BoxDrawingChar::TopT.into(), nx, y),
            '7' => pixel(BoxDrawingChar::TopRightCorner.into(), nx, y),
            '|' => pixel(BoxDrawingChar::Vertical.into(), nx, y),
            'p' => pixel(BoxDrawingChar::LeftT.into(), nx, y),
            '+' => pixel(BoxDrawingChar::MiddleT.into(), nx, y),
            'd' => pixel(BoxDrawingChar::RightT.into(), nx, y),
            'l' => pixel(BoxDrawingChar::BottomLeftCorner.into(), nx, y),
            't' => pixel(BoxDrawingChar::BottomT.into(), nx, y),
            'j' => pixel(BoxDrawingChar::BottomRightCorner.into(), nx, y),

            _ => pixel(c, nx, y),
        }
        nx += 1;
    }
}

/// Draw a box using ASCII box-drawing characters.
///
/// # Errors
///
/// If the given line is non-orthogonal, returns an error.
pub fn ascii_box(x1: u32, y1: u32, x2: u32, y2: u32) -> Result<(), NonOrthogonal> {
    orth_line(BoxDrawingChar::Horizontal.into(), x1 + 1, y1, x2 - 1, y1)?;
    orth_line(BoxDrawingChar::Horizontal.into(), x1 + 1, y2, x2 - 1, y2)?;
    orth_line(BoxDrawingChar::Vertical.into(), x1, y1 + 1, x1, y2 - 1)?;
    orth_line(BoxDrawingChar::Vertical.into(), x2, y1 + 1, x2, y2 - 1)?;

    pixel(BoxDrawingChar::TopLeftCorner.into(), x1, y1);
    pixel(BoxDrawingChar::TopRightCorner.into(), x2, y1);
    pixel(BoxDrawingChar::BottomLeftCorner.into(), x1, y2);
    pixel(BoxDrawingChar::BottomRightCorner.into(), x2, y2);

    Ok(())
}

/// Draw a filled rectangle onto the screen.
///
/// # Errors
///
/// If the given line is non-orthogonal, returns an error.
pub fn rect_fill(c: char, x1: u32, y1: u32, x2: u32, y2: u32) -> Result<(), NonOrthogonal> {
    let mut y = y1;
    while y != y2 {
        orth_line(c, x1, y, x2, y)?;
        y += 1;
    }

    Ok(())
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
