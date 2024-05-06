/// Prints text, moving the cursor to the start of the next line.
///
/// Does not print a newline, and thus does not flush line buffering.
#[macro_export]
macro_rules! println {
    ( $($format:tt)* ) => {
        ::std::print!($($format)*);
        $crate::goto::start();
        $crate::goto::down(1);
    };
}
