//! A utility for command-line drawing.
//!
//! Many utilities are at the top-level; however, there are a couple of modules:
//!  - [`clear`]: Functions for clearing the screen or sections thereof.
//!  - [`color`]: Functions for setting the foreground/background color.
//!  - [`goto`]: Functions for moving the cursor around the screen.
//!  - [`read`]: Functions for reading from stdin. Must specify feature `input` to use.

use std::io::{stdout, Write};

#[cfg(feature = "input")]
pub use console::Key;

mod line;
use line::LineIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodError {
    InvalidOrthoLine(u32, u32, u32, u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    fn from(ch: BoxDrawingChar) -> char {
        match ch {
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

/// Utilities for setting and resetting color.
pub mod color {
    use crate::escape;

    /// Set foreground color, using 24-bit true color (not supported on all terminals).
    pub fn tc_fg(r: u8, g: u8, b: u8) {
        escape(format!("38;2;{r};{g};{b}m"));
    }

    /// Set background color, using 24-bit true color (not supported on all terminals).
    pub fn tc_bg(r: u8, g: u8, b: u8) {
        escape(format!("48;2;{r};{g};{b}m"));
    }

    /// Set foreground color, using 8-bit color.
    pub fn fg(color: u8) {
        escape(format!("38;5;{color}m"));
    }

    /// Set background color, using 8-bit color.
    pub fn bg(color: u8) {
        escape(format!("48;5;{color}m"));
    }

    /// Reset foreground color.
    pub fn de_fg() {
        escape("39m");
    }

    /// Reset background color.
    pub fn de_bg() {
        escape("49m");
    }

    /// Remove all color modifiers.
    pub fn de() {
        escape("0m");
    }
}

/// Utilities for clearing the screen.
pub mod clear {
    use crate::escape;

    /// Clear the screen (full clear, not scroll).
    pub fn all() {
        escape("2J");
    }

    /// Clear the current line.
    pub fn line() {
        escape("2K");
    }

    /// Clear a portion of the screen. *Note:* will clear using the current background color.
    pub fn rect(x1: u32, y1: u32, x2: u32, y2: u32) -> Result<(), crate::CodError> {
        crate::rect_fill(' ', x1, y1, x2, y2)
    }
}

/// Draw a single character onto the screen.
pub fn pixel(c: char, x: u32, y: u32) {
    escape(format!("{};{}H{}", y + 1, x + 1, c));
}

/// Draw an orthogonal line to the screen.
pub fn orth_line(c: char, x1: u32, y1: u32, x2: u32, y2: u32) -> Result<(), CodError> {
    if x1 != x2 && y1 != y2 {
        return Err(CodError::InvalidOrthoLine(x1, y1, x2, y2));
    }

    if x1 != x2 {
        let mut x = x1.min(x2);

        while x != x1.max(x2) + 1 {
            pixel(c, x, y1);

            x += 1;
        }
    } else {
        let mut y = y1.min(y2);

        while y != y1.max(y2) + 1 {
            pixel(c, x1, y);

            y += 1;
        }
    }

    Ok(())
}

/// Draw a line onto the screen.
pub fn line(c: char, x1: u32, y1: u32, x2: u32, y2: u32) {
    if x1 == x2 || y1 == y2 {
        orth_line(c, x1, x2, y1, y2).unwrap();
        return;
    }

    for (x, y) in LineIter::new(x1, y1, x2, y2) {
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
pub fn rect(c: char, x1: u32, y1: u32, x2: u32, y2: u32) -> Result<(), CodError> {
    orth_line(c, x1, y1, x1, y2)?;
    orth_line(c, x1, y1, x2, y1)?;
    orth_line(c, x2, y2, x1, y2)?;
    orth_line(c, x2, y2, x2, y1)?;

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoxChars {
    pub horizontal: char,
    pub vertical: char,
    pub corner: char,
}

/// Draw a rectangle using a set of characters.
pub fn rect_lines(chars: BoxChars, x1: u32, y1: u32, x2: u32, y2: u32) -> Result<(), CodError> {
    orth_line(chars.horizontal, x1, y1, x2, y1)?;
    orth_line(chars.horizontal, x1, y2, x2, y2)?;
    orth_line(chars.vertical, x1, y1, x1, y2)?;
    orth_line(chars.vertical, x2, y1, x2, y2)?;
    pixel(chars.corner, x1, y1);
    pixel(chars.corner, x1, y2);
    pixel(chars.corner, x2, y1);
    pixel(chars.corner, x2, y2);

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
pub fn ascii_box(x1: u32, y1: u32, x2: u32, y2: u32) -> Result<(), CodError> {
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
pub fn rect_fill(c: char, x1: u32, y1: u32, x2: u32, y2: u32) -> Result<(), CodError> {
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

/// Utilities for moving the cursor.
pub mod goto {
    use crate::escape;

    /// Move the cursor up.
    pub fn up(y: u32) {
        if y == 0 {
            return;
        }
        escape(format!("{y}A"));
    }

    /// Move the cursor down.
    pub fn down(y: u32) {
        if y == 0 {
            return;
        }
        escape(format!("{y}B"));
    }

    /// Move the cursor left.
    pub fn left(x: u32) {
        if x == 0 {
            return;
        }
        escape(format!("{x}D"));
    }

    /// Move the cursor right.
    pub fn right(x: u32) {
        if x == 0 {
            return;
        }
        escape(format!("{x}C"));
    }

    /// Set cursor to a specific position.
    pub fn pos(x: u32, y: u32) {
        escape(format!("{};{}H", y + 1, x + 1));
    }

    /// Move the cursor to the top of screen.
    pub fn home() {
        pos(0, 0);
    }

    /// Move the cursor to the bottom of the screen.
    pub fn bot() {
        pos(0, 9998);
    }
}

/// Utilities for modifying the look of the text.
pub mod style {
    use crate::escape;

    /// Enable bold.
    pub fn bold() {
        escape("1m");
    }

    /// Enable faint.
    pub fn faint() {
        escape("4m");
    }

    /// Enable italics.
    pub fn italic() {
        escape("3m");
    }

    /// Enable underline.
    pub fn underline() {
        escape("4m");
    }

    /// Enable strikethrough.
    pub fn strike() {
        escape("9m");
    }

    /// Disable all style attributes.
    pub fn de() {
        escape("22");
        escape("23");
        escape("24");
        escape("29");
    }
}

/// Flush to stdout.
pub fn flush() {
    stdout().flush().expect("Failed to flush stdout");
}

/// Utilities for reading from stdin.
#[cfg(feature = "input")]
pub mod read {
    use console::{Key, Term};

    /// Read a single key from stdin.
    pub fn key() -> Option<Key> {
        let term = Term::stdout();
        term.read_key().ok()
    }

    /// Read a line from stdin.
    pub fn line() -> Option<String> {
        let term = Term::stdout();
        term.read_line().ok()
    }
}
