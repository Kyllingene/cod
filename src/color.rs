//! Utilities for setting and resetting color.
//!
//! By default, the feature `color_stack` is enabled. This adds a global,
//! static stack to keep track of coloring. These utilities can also be used
//! directly via the following functions:
//! - [`push_fg`]
//! - [`push_bg`]
//! - [`push_tc_fg`]
//! - [`push_tc_bg`]
//! - [`pop_fg`]
//! - [`pop_bg`]
//!
//! Without `color_stack`, the functions in [`with`] don't nest well: when the
//! inner one exits, the color will be reset to normal, rather than continue
//! the color that the outer function set.

use crate::escape;

#[cfg(feature = "color_stack")]
pub use stack::{
    bg::pop as pop_bg, bg::push::bg as push_bg, bg::push::tc_bg as push_tc_bg, fg::pop as pop_fg,
    fg::push::fg as push_fg, fg::push::tc_fg as push_tc_fg,
};

#[cfg(feature = "color_stack")]
#[allow(clippy::missing_panics_doc)]
mod stack {
    use super::{bg, fg, tc_bg, tc_fg};
    use std::sync::Mutex;

    fn init_stack<T>() -> Mutex<Vec<T>> {
        Mutex::new(Vec::new())
    }

    #[derive(Clone, Copy)]
    enum Color {
        C(u8),
        Rgb(u8, u8, u8),
    }

    impl Color {
        fn fg(self) {
            match self {
                Self::C(x) => fg(x),
                Self::Rgb(r, g, b) => tc_fg(r, g, b),
            }
        }

        fn bg(self) {
            match self {
                Self::C(x) => bg(x),
                Self::Rgb(r, g, b) => tc_bg(r, g, b),
            }
        }
    }

    pub mod fg {
        use std::sync::{Mutex, OnceLock};
        static FG_COLOR_STACK: OnceLock<Mutex<Vec<Color>>> = OnceLock::new();

        use super::{init_stack, Color};
        use crate::color::de;

        pub mod push {
            use crate::color::stack::{init_stack, Color};

            use super::FG_COLOR_STACK;

            /// Pushes a color onto the foreground color stack.
            pub fn fg(c: u8) {
                FG_COLOR_STACK
                    .get_or_init(init_stack)
                    .lock()
                    .unwrap()
                    .push(Color::C(c));
            }

            /// Pushes an RGB color onto the foreground color stack.
            pub fn tc_fg(r: u8, g: u8, b: u8) {
                FG_COLOR_STACK
                    .get_or_init(init_stack)
                    .lock()
                    .unwrap()
                    .push(Color::Rgb(r, g, b));
            }
        }

        /// Pops a color off of the foreground color stack.
        pub fn pop() {
            let mut stack = FG_COLOR_STACK.get_or_init(init_stack).lock().unwrap();

            stack.pop();
            if let Some(c) = stack.last() {
                c.fg();
            } else {
                de::fg();
            }
        }
    }

    pub mod bg {
        use std::sync::{Mutex, OnceLock};
        static BG_COLOR_STACK: OnceLock<Mutex<Vec<Color>>> = OnceLock::new();

        use super::{init_stack, Color};
        use crate::color::de;

        pub mod push {
            use crate::color::stack::{init_stack, Color};

            use super::BG_COLOR_STACK;

            /// Pushes a color onto the background color stack.
            pub fn bg(c: u8) {
                BG_COLOR_STACK
                    .get_or_init(init_stack)
                    .lock()
                    .unwrap()
                    .push(Color::C(c));
            }

            /// Pushes an RGB color onto the background color stack.
            pub fn tc_bg(r: u8, g: u8, b: u8) {
                BG_COLOR_STACK
                    .get_or_init(init_stack)
                    .lock()
                    .unwrap()
                    .push(Color::Rgb(r, g, b));
            }
        }

        /// Pops a color off of the background color stack.
        pub fn pop() {
            let mut stack = BG_COLOR_STACK.get_or_init(init_stack).lock().unwrap();

            stack.pop();
            if let Some(c) = stack.last() {
                c.bg();
            } else {
                de::bg();
            }
        }
    }
}

macro_rules! do_color {
    ( $( $color:ident, $doc:literal, [ $( $arg:ident : $typ:ty ),+ ], $fmt:literal ),+ $(,)? ) => {
        $(
            /// Set the
            #[doc = $doc]
            pub fn $color($($arg: $typ,)+) {
                escape(format!(concat!($fmt, "m"), $($arg,)+));
            }
        )+
    };
}

do_color![
    fg, "foreground color.", [color: u8], "38;5;{}",
    bg, "background color.", [color: u8], "48;5;{}",
    tc_fg, "foreground color, using true-color.", [r: u8, g: u8, b: u8], "38;2;{};{};{}",
    tc_bg, "background color, using true-color.", [r: u8, g: u8, b: u8], "48;2;{};{};{}",
];

/// Decolor your text.
pub mod de {
    use crate::escape;

    /// Reset the foreground color.
    pub fn fg() {
        escape("39m");
    }

    /// Reset the background color.
    pub fn bg() {
        escape("49m");
    }

    /// Reset all color.
    pub fn all() {
        fg();
        bg();
    }
}

/// Color your text through closures.
///
/// Example:
///
/// ```
/// # use cod::color;
/// color::fg(1);
/// color::with::bg(4, || {
///     println!("I'm red and blue!");
/// });
///
/// println!("I'm red!");
/// ```
pub mod with {
    macro_rules! with_color {
        ( $( $color:ident, $de:ident, $doc:literal, [ $( $arg:ident : $typ:ty ),+ ] ),+ $(,)? ) => {
            $(
                /// Set the
                #[doc = $doc]
                /// then run the function, then reset it.
                pub fn $color($($arg: $typ,)+ f: impl FnOnce()) {
                    super::$color($($arg,)+);
                    #[cfg(feature = "color_stack")]
                    super::stack::$de::push::$color($($arg,)+);
                    (f)();
                    #[cfg(feature = "color_stack")]
                    super::stack::$de::pop();
                    #[cfg(not(feature = "color_stack"))]
                    super::de::$de();
                }
            )+
        };
    }

    with_color![
        fg, fg, "foreground color,", [color: u8],
        bg, bg, "background color,", [color: u8],
        tc_fg, fg, "foreground color (using true-color),", [r: u8, g: u8, b: u8],
        tc_bg, bg, "background color (using true-color),", [r: u8, g: u8, b: u8],
    ];
}
