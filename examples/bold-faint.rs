//! Use to test both faint and bold at the same time. On some terminals, like
//! XTerm, they are separate; on others, like Alacritty, bold takes precedence,
//! unless it's colored. In short, behavior differs.

use cod::prelude::*;

fn main() {
    let _g = guard::Reset;

    style::with::bold(|| println!("This is just bold"));
    style::with::faint(|| println!("This is just faint"));
    style::with::bold(|| style::with::faint(|| println!("This is bold, then faint")));
    style::with::faint(|| style::with::bold(|| println!("This is faint, then bold")));

    for color in [0, 1, 2, 3, 4, 5, 6, 7] {
        println!();
        color::with::fg(color, || {
            style::with::bold(|| println!("This is just bold"));
            style::with::faint(|| println!("This is just faint"));
            style::with::bold(|| style::with::faint(|| println!("This is bold, then faint")));
            style::with::faint(|| style::with::bold(|| println!("This is faint, then bold")));
        });
    }
}

