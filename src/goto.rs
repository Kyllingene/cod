//! Utilities for moving the cursor.

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
