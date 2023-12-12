use cod::prelude::*;

fn main() {
    let _g = guard::Reset;

    style::bold();
    print!("This is ");

    style::with::italic(|| {
        print!("italic and bold text");
    });

    println!(", but this is just bold.");
}
