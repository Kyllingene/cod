//! Utilities for drawing various rectangles and boxes.
use crate::{orth_line, pixel, NonOrthogonal};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BoxDrawingChar {
    Horizontal,
    Vertical,

    TopLeftCorner,
    TopRightCorner,
    BottomLeftCorner,
    BottomRightCorner,
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
        }
    }
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

/// Draw a rectangle onto the screen using just the given characters.
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
pub fn with(chars: Chars, x1: u32, y1: u32, x2: u32, y2: u32) -> Result<(), NonOrthogonal> {
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

/// Draw a box using ASCII box-drawing characters.
///
/// # Errors
///
/// If the given line is non-orthogonal, returns an error.
pub fn ascii(x1: u32, y1: u32, x2: u32, y2: u32) -> Result<(), NonOrthogonal> {
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
