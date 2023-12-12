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
    crate::rect(' ', x1, y1, x2, y2)
}
