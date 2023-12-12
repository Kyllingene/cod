use cod::prelude::*;

fn main() {
    let messages = [
        (1, "Failure"),
        (2, "Success"),
        (3, "Warning"),
        (4, "Information"),
    ];

    for (color, message) in messages {
        color::with::fg(color, || {
            print!("{message}");
        });

        println!("!");
    }
}
