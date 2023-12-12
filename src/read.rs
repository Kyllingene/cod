//! Utilities for reading from stdin.

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
