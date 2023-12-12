//! Provides [`Reset`], a simple type to make sure the terminal gets reset
//! when your program exits.

/// When dropped, resets all style and color attributes. Can be used to ensure
/// the terminal is reset before exiting the program or function, or you could
/// use `drop(Reset)` to guarantee reset all styles.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reset;

impl std::ops::Drop for Reset {
    fn drop(&mut self) {
        crate::style::de::all();
        crate::color::de::all();
    }
}
