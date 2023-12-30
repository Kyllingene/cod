use cod::prelude::*;

fn main() {
    color::with::fg(1, || {
        println!("Red!");
        color::with::fg(2, || println!("Green!"));
        println!("Red again!");
    });
}
