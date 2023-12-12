use crate::escape;

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
                    (f)();
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
