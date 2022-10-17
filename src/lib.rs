use std::cmp::{max, min};
use std::io::{stdout, Write};
use std::{thread, time};

/// Print an escape sequence
fn escape<T: std::fmt::Display>(code: T) {
    print!("{}[{}", 27 as char, code);
}

/// Set foreground color, using 24-bit true color (not supported on all terminals)
pub fn tc_color_fg(r: u8, g: u8, b: u8) {
    escape(format!("38;2;{};{};{}", r, g, b));
}

/// Set background color, using 24-bit true color (not supported on all terminals)
pub fn tc_color_bg(r: u8, g: u8, b: u8) {
    escape(format!("48;2;{};{};{}", r, g, b));
}

/// Set foreground color, using 8-bit color
pub fn color_fg(color: u8) {
    escape(format!("38;5;{}m", color));
}

/// Set background color, using 8-bit color
pub fn color_bg(color: u8) {
    escape(format!("48;5;{}m", color));
}

/// Remove all color modifiers
pub fn decolor() {
    escape("0m");
}

/// Clear the screen (full clear, not scroll)
pub fn clear() {
    print!("{}c", 27 as char);
}

/// Draw a single character onto the screen
pub fn pixel(c: char, x: u32, y: u32) {
    escape(format!("{};{}H{}", y, x, c));
    stdout().flush().unwrap();
}

/// Draw an orthogonal line to the screen
pub fn orth_line(c: char, x1: u32, y1: u32, x2: u32, y2: u32) {
    if x1 != x2 && y1 != y2 {
        // TODO: replace with error handling
        panic!("Cannot draw non-ortho lines with orth-line");
    }

    if x1 != x2 {
        let mut x = min(x1, x2);

        while x != max(x1, x2) + 1 {
            pixel(c, x, y1);

            x += 1;
        }
    } else {
        let mut y = min(y1, y2);

        while y != max(y1, y2) + 1 {
            pixel(c, x1, y);

            y += 1;
        }
    }
}

/// Draw a line onto the screen
pub fn line(c: char, x1: u32, y1: u32, x2: u32, y2: u32) {
    if x1 == x2 || y1 == y2 {
        orth_line(c, x1, x2, y1, y2);
        return;
    }

    let mut dx: i32 = (x2 - x1) as i32;
    let mut dy: i32 = (y2 - y1) as i32;

    let sx = if (x2 as i32) - (x1 as i32) > 0 { 1 } else { -1 };
    let sy = if (y2 as i32) - (y1 as i32) > 0 { 1 } else { -1 };

    let xx;
    let xy;
    let yx;
    let yy;
    if dx > dy {
        xx = sx;
        xy = 0i32;
        yx = 0i32;
        yy = sy;
    } else {
        std::mem::swap(&mut dx, &mut dy);
        xx = 0i32;
        xy = sy;
        yx = sx;
        yy = 0i32;
    }

    let mut err = ((dy << 1) - dx) as i32;

    let mut x = 0;
    let mut y = 0;

    while x <= dx {
        pixel(
            c,
            ((x1 as i32) + x * xx + y * yx) as u32,
            ((y1 as i32) + x * xy + y * yy) as u32,
        );

        if err >= 0 {
            y += 1;
            err -= (dx as i32) << 1;
        }

        err += (dy as i32) << 1;
        x += 1;
    }
}

/// Draw a "texture" onto the screen
pub fn blit(src: &Vec<Vec<char>>, sx: u32, sy: u32) {
    let mut x = sx;
    let mut y = sy;
    for row in src {
        for c in row {
            pixel(*c, x, y);
            x += 1;
        }
        x = sx;
        y += 1;
    }
}

/// Draw a "texture" onto the screen
pub fn blit_str(src: &String, x: u32, y: u32) {
    let split = String::from(src)
        .split('\n')
        .map(|s| s.chars())
        .map(|c| c.collect())
        .collect();
    blit(&split, x, y);
}

/// Draw a "texture" onto the screen
pub fn blit_vstrs(src: &Vec<String>, x: u32, y: u32) {
    let vec = src.iter().map(|s| s.chars()).map(|c| c.collect()).collect();
    blit(&vec, x, y);
}

/// Draw a rectangle onto the screen
pub fn rect(c: char, x1: u32, y1: u32, x2: u32, y2: u32) {
    orth_line(c, x1, y1, x1, y2);
    orth_line(c, x1, y1, x2, y1);
    orth_line(c, x2, y2, x1, y2);
    orth_line(c, x2, y2, x2, y1);
}

/// Draw a filled rectangle onto the screen
pub fn rect_fill(c: char, x1: u32, y1: u32, x2: u32, y2: u32) {
    let mut y = y1;
    while y != y2 {
        orth_line(c, x1, y, x2, y);
        y += 1;
    }
}

/// Draw a triangle onto the screen
pub fn triangle(c: char, x1: u32, y1: u32, x2: u32, y2: u32, x3: u32, y3: u32) {
    line(c, x1, y1, x2, y2);
    line(c, x1, y1, x3, y3);
    line(c, x3, y3, x2, y2);
}

/// Draw a filled triangle onto the screen
pub fn triangle_fill(c: char, x1: u32, y1: u32, x2: u32, y2: u32, x3: u32, y3: u32) {
    unimplemented!();
}

/// Draw text onto the screen (non-wrapping)
pub fn text(s: String, ox: u32, y: u32) {
    let mut x = ox;

    for c in s.chars() {
        pixel(c, x, y);
        x += 1;
    }
}

/// Set cursor to position
pub fn goto(x: u32, y: u32) {
    escape(format!("{};{}H", y, x));
}

/// Put cursor to top of screen
pub fn home() {
    goto(1, 1);
    stdout().flush().unwrap();
}

/// Put cursor to the bottom of the screen
pub fn bot() {
    goto(1, 9999);
    stdout().flush().unwrap();
}

// TODO: probably remove
/// Pause for a certain amount of seconds
pub fn sleep(seconds: f32) {
    let secs = seconds as u64;
    let mils = (seconds % 1.0 * 1000.0 * 1000000.0) as u32;
    thread::sleep(time::Duration::new(secs, mils));
}
