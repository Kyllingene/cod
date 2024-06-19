//! Utilities for handling the terminal.
//!
//! Only enabled on feature `crossterm`.

/// The style of the cursor, used with [`set_cursor_style`].
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum CursorStyle {
    /// Default cursor shape configured by the user.
    #[default]
    DefaultUserShape,
    /// A blinking block cursor shape (■).
    BlinkingBlock,
    /// A non blinking block cursor shape (inverse of `BlinkingBlock`).
    SteadyBlock,
    /// A blinking underscore cursor shape(_).
    BlinkingUnderScore,
    /// A non blinking underscore cursor shape (inverse of `BlinkingUnderScore`).
    SteadyUnderScore,
    /// A blinking cursor bar shape (|)
    BlinkingBar,
    /// A steady cursor bar shape (inverse of `BlinkingBar`).
    SteadyBar,
}

/// Tries to get the terminal size in columns and rows.
///
/// If you'd like sensible defaults on failure, see [`size_or`].
///
/// Only enabled on feature `crossterm`.
#[cfg(any(feature = "crossterm", doc))]
#[allow(clippy::must_use_candidate)]
pub fn size() -> Option<(u32, u32)> {
    crossterm::terminal::size()
        .ok()
        .map(|(cols, rows)| (u32::from(cols), u32::from(rows)))
}

/// Returns the terminal size in columns and rows.
///
/// If this fails to get the size, instead returns sensible defaults (80x24).
///
/// Only enabled on feature `crossterm`.
#[cfg(any(feature = "crossterm", doc))]
#[allow(clippy::must_use_candidate)]
pub fn size_or() -> (u32, u32) {
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
    match style {
        CursorStyle::DefaultUserShape => print!("\x1b[0 q"),
        CursorStyle::BlinkingBlock => print!("\x1b[1 q"),
        CursorStyle::SteadyBlock => print!("\x1b[2 q"),
        CursorStyle::BlinkingUnderScore => print!("\x1b[3 q"),
        CursorStyle::SteadyUnderScore => print!("\x1b[4 q"),
        CursorStyle::BlinkingBar => print!("\x1b[5 q"),
        CursorStyle::SteadyBar => print!("\x1b[6 q"),
    }
}

/// Switch to the secondary screen.
///
/// Use [`primary_screen`] to swap back.
pub fn secondary_screen() {
    print!("\x1b[?1049h");
}

/// Switch to the primary (default) screen.
///
/// Use [`secondary_screen`] to swap back.
pub fn primary_screen() {
    print!("\x1b[?1049l");
}

/// Enable raw mode for the terminal.
///
/// Disable with [`disable_raw_mode`].
///
/// # Panics
///
/// If this fails to enable raw mode, panics with the message "failed to enable
/// raw mode".
#[cfg(any(feature = "crossterm", doc))]
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
#[cfg(any(feature = "crossterm", doc))]
pub fn disable_raw_mode() {
    crossterm::terminal::disable_raw_mode().expect("failed to disable raw mode");
}

/// A simple utility to guarantee raw mode is exited.
///
/// Exits raw mode when dropped (e.g. on program exit).
#[cfg(any(feature = "crossterm", doc))]
#[must_use = "does nothing unless stored, consider `let _guard = ...`"]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RawModeGuard;

#[cfg(any(feature = "crossterm", doc))]
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

#[cfg(any(feature = "crossterm", doc))]
impl Drop for RawModeGuard {
    fn drop(&mut self) {
        disable_raw_mode();
    }
}
