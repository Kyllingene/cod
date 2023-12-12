use cod::prelude::*;

fn main() {
    style::bold();
    print!("This is ");

    style::with::italic(|| {
        print!("italic and bold text");
    });

    println!(", but this is just bold.");

    // this is necessary, otherwise shells like bash will get affected!
    // this is why `with::` functions are much better.
    style::de::bold();
}
