//! Utilities for handling the terminal.
//!
//! Only enabled on feature `crossterm`.

pub use crossterm::cursor::SetCursorStyle as CursorStyle;

/// Returns the terminal size in columns and rows.
///
/// If this fails to get the size, instead returns sensible defaults (80x24).
#[allow(clippy::must_use_candidate)]
pub fn size() -> (u32, u32) {
    crossterm::terminal::size()
        .map(|(cols, rows)| (u32::from(cols), u32::from(rows)))
        .unwrap_or((80, 24))
}

/// Changes the cursor style.
///
/// # Panics
///
/// If this fails to set the cursor style, panics with the message "failed to
/// change cursor style".
pub fn set_cursor_style(style: CursorStyle) {
    crossterm::execute!(std::io::stdout(), style).expect("failed to change cursor style");
}

/// Enable raw mode for the terminal.
///
/// Disable with [`disable_raw_mode`].
///
/// # Panics
///
/// If this fails to enable raw mode, panics with the message "failed to enable
/// raw mode".
pub fn enable_raw_mode() {
    crossterm::terminal::enable_raw_mode().expect("failed to enable raw mode");
}

/// Disable raw mode for the terminal.
///
/// Enable with [`enable_raw_mode`].
///
/// # Panics
///
/// If this fails to disable raw mode, panics with the message "failed to
/// disable raw mode".
pub fn disable_raw_mode() {
    crossterm::terminal::disable_raw_mode().expect("failed to disable raw mode");
}

/// A simple utility to guarantee raw mode is exited.
///
/// Exits raw mode when dropped (e.g. on program exit).
#[must_use = "does nothing unless stored, consider `let _guard = ...`"]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RawModeGuard;

impl RawModeGuard {
    /// Creates a new guard, entering raw mode in the process.
    ///
    /// If you don't wish to enter raw mode, instead construct the guard
    /// directly.
    pub fn enter() -> Self {
        enable_raw_mode();
        Self
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        disable_raw_mode();
    }
}
