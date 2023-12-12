/// Utilities for drawing various rectangles and boxes.

use crate::{NonOrthogonal, pixel, orth_line};

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

/// Characters to use when calling [`with`].
///
/// Note that `corner` will be used on all four corners.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Chars {
    /// Horizontal lines.
    pub horizontal: char,
    /// Vertical lines.
    pub vertical: char,
    /// Corner pixels.
    pub corner: char,
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

/// Draw a rectangle onto the screen.
///
/// # Errors
///
/// If the given line is non-orthogonal, returns an error.
pub fn line(c: char, x1: u32, y1: u32, x2: u32, y2: u32) -> Result<(), NonOrthogonal> {
    orth_line(c, x1, y1, x1, y2)?;
    orth_line(c, x1, y1, x2, y1)?;
    orth_line(c, x2, y2, x1, y2)?;
    orth_line(c, x2, y2, x2, y1)?;

    Ok(())
}

/// Draw a rectangle using a given set of characters, via [`Chars`].
///
/// # Errors
///
/// If the given line is non-orthogonal, returns an error.
pub fn with(
    chars: Chars,
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
) -> Result<(), NonOrthogonal> {
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
pub fn fill(c: char, x1: u32, y1: u32, x2: u32, y2: u32) -> Result<(), NonOrthogonal> {
    let mut y = y1;
    while y != y2 {
        orth_line(c, x1, y, x2, y)?;
        y += 1;
    }

    Ok(())
}
