//! Utilities for reading from stdin.
#![allow(clippy::must_use_candidate)]

use crossterm::event::{Event, KeyEventKind};
pub use crossterm::event::{KeyCode, KeyEvent, KeyEventState as KeyState, KeyModifiers};

/// Read a single key from stdin.
pub fn key() -> Option<KeyEvent> {
    match crossterm::event::read().ok()? {
        Event::Key(ev) if ev.kind != KeyEventKind::Release => Some(ev),
        _ => None,
    }
}

/// Read a single key from stdin, discarding key repeats.
pub fn key_once() -> Option<KeyEvent> {
    match crossterm::event::read().ok()? {
        Event::Key(ev) if ev.kind == KeyEventKind::Press => Some(ev),
        _ => None,
    }
}

/// Read a line from stdin.
///
/// Is *not* a full line editor.
pub fn line() -> String {
    let mut line = String::new();
    while let Some(KeyEvent { code, .. }) = key() {
        match code {
            KeyCode::Enter => {
                break;
            }
            KeyCode::Char(c) => {
                line.push(c);
            }
            _ => {}
        }
    }

    line
}
