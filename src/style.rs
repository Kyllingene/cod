//! Utilities for modifying the look of the text.
//!
//! Note that faint and bold are mutually exclusive on some terminals, thus
//! [`de::weight`] resets both simultaneously. This behavior extends to
//! [`with::bold`] and [`with::faint`].

use crate::escape;

macro_rules! do_style {
    ( $( $style:ident: $code:tt ),+ ) => {
        $(
            /// Enable
            #[doc = concat!(stringify!($style), ".")]
            pub fn $style() {
                escape(concat!(stringify!($code), "m"));
            }
        )+
    };
}

do_style!(bold: 1, faint: 2, italic: 3, underline: 4, strike: 9);

/// Reset styling.
pub mod de {
    use crate::escape;

    macro_rules! de_style {
        ( $( $style:ident: $code:tt ),+ ) => {
            $(
                /// Disable
                #[doc = concat!(stringify!($style), ".")]
                pub fn $style() {
                    escape(concat!(stringify!($code), "m"));
                }
            )+

            /// Disable all style attributes.
            pub fn all() {
                $($style();)+
                weight();
            }
        };
    }

    de_style!(italic: 23, underline: 24, strike: 29);

    /// Disables both bold and faint styling.
    ///
    /// See module documentation for why.
    pub fn weight() {
        escape("22m");
    }
}

/// Style your text through closures.
///
/// Example:
///
/// ```
/// # use cod::style;
/// style::italic();
/// style::with::bold(|| {
///     println!("I'm italic and bold!");
/// });
///
/// println!("I'm italic!");
/// ```
pub mod with {
    macro_rules! with_style {
        ( $( $style:ident: $de:ident ),+ ) => {
            $(
                /// Enable
                #[doc = concat!(stringify!($style), ",")]
                /// then run the function, then disable it again.
                pub fn $style(f: impl FnOnce()) {
                    super::$style();
                    (f)();
                    super::de::$de();
                }
            )+
        };
    }

    with_style![
        bold: weight,
        faint: weight,
        italic: italic,
        underline: underline,
        strike: strike
    ];
}
