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
            }
        };
    }

    de_style!(bold: 22, faint: 22, italic: 23, underline: 24, strike: 29);
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
        ( $( $style:ident ),+ ) => {
            $(
                /// Enable
                #[doc = concat!(stringify!($style), ",")]
                /// then run the function, then disable it again.
                pub fn $style(f: impl FnOnce()) {
                    super::$style();
                    (f)();
                    super::de::$style();
                }
            )+
        };
    }

    with_style!(bold, faint, italic, underline, strike);
}
