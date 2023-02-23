use std::cmp::{max, min};
use std::io::{stdout, Write};

#[cfg(feature = "input")]
use console::Term;

#[cfg(feature = "input")]
pub use console::Key;

enum BoxDrawingChar {
    Horizontal,
    Vertical,

    TopLeftCorner,
    TopRightCorner,
    BottomLeftCorner,
    BottomRightCorner,

    TopT,
    LeftT,
    RightT,
    BottomT,
    MiddleT,
}

impl From<BoxDrawingChar> for char {
    fn from(val: BoxDrawingChar) -> char {
        match val {
            BoxDrawingChar::Horizontal => '\u{2550}',
            BoxDrawingChar::Vertical => '\u{2551}',

            BoxDrawingChar::TopLeftCorner => '\u{2554}',
            BoxDrawingChar::TopRightCorner => '\u{2557}',
            BoxDrawingChar::BottomLeftCorner => '\u{255A}',
            BoxDrawingChar::BottomRightCorner => '\u{255D}',

            BoxDrawingChar::TopT => '\u{2566}',
            BoxDrawingChar::LeftT => '\u{2560}',
            BoxDrawingChar::RightT => '\u{2563}',
            BoxDrawingChar::BottomT => '\u{2569}',
            BoxDrawingChar::MiddleT => '\u{256C}',
        }
    }
}

/// Print an escape sequence.
fn escape<T: std::fmt::Display>(code: T) {
    print!("{}[{}", 27 as char, code);
}

/// Set foreground color, using 24-bit true color (not supported on all terminals).
pub fn tc_color_fg(r: u8, g: u8, b: u8) {
    escape(format!("38;2;{r};{g};{b}"));
}

/// Set background color, using 24-bit true color (not supported on all terminals).
pub fn tc_color_bg(r: u8, g: u8, b: u8) {
    escape(format!("48;2;{r};{g};{b}"));
}

/// Set foreground color, using 8-bit color.
pub fn color_fg(color: u8) {
    escape(format!("38;5;{color}m"));
}

/// Set background color, using 8-bit color.
pub fn color_bg(color: u8) {
    escape(format!("48;5;{color}m"));
}

/// Remove all color modifiers.
pub fn decolor() {
    escape("0m");
}

/// Clear the screen (full clear, not scroll).
pub fn clear() {
    print!("{}c", 27 as char);
}

/// Clear a portion of the screen.
pub fn erase(x1: u32, y1: u32, x2: u32, y2: u32) -> Result<(), &'static str> {
    rect(' ', x1, y1, x2, y2)
}

/// Draw a single character onto the screen.
pub fn pixel(c: char, x: u32, y: u32) {
    escape(format!("{};{}H{}", y + 1, x + 1, c));
}

/// Draw an orthogonal line to the screen.
pub fn orth_line(c: char, x1: u32, y1: u32, x2: u32, y2: u32) -> Result<(), &'static str> {
    if x1 != x2 && y1 != y2 {
        return Err("Cannot draw non-ortho lines with orth-line");
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

    Ok(())
}

/// Draw a line onto the screen.
pub fn line(c: char, x1: u32, y1: u32, x2: u32, y2: u32) -> Result<(), &'static str> {
    if x1 == x2 || y1 == y2 {
        orth_line(c, x1, x2, y1, y2)?;
        return Ok(());
    }

    let mut dx: i32 = (x2 - x1) as i32;
    let mut dy: i32 = (y2 - y1) as i32;

    let sx = if x2 - x1 > 0 { 1 } else { -1 };
    let sy = if y2 - y1 > 0 { 1 } else { -1 };

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

    let mut err = (dy << 1) - dx;

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
            err -= dx << 1;
        }

        err += dy << 1;
        x += 1;
    }

    Ok(())
}

/// Draw a "texture" onto the screen.
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

/// Draw a "texture" onto the screen.
pub fn blit_str(src: &String, x: u32, y: u32) {
    let split = String::from(src)
        .split('\n')
        .map(|s| s.chars())
        .map(|c| c.collect())
        .collect();
    blit(&split, x, y);
}

/// Draw a "texture" onto the screen.
pub fn blit_vstrs(src: &[String], x: u32, y: u32) {
    let vec = src.iter().map(|s| s.chars()).map(|c| c.collect()).collect();
    blit(&vec, x, y);
}

/// Draw a rectangle onto the screen.
pub fn rect(c: char, x1: u32, y1: u32, x2: u32, y2: u32) -> Result<(), &'static str> {
    orth_line(c, x1, y1, x1, y2)?;
    orth_line(c, x1, y1, x2, y1)?;
    orth_line(c, x2, y2, x1, y2)?;
    orth_line(c, x2, y2, x2, y1)?;

    Ok(())
}

/// Print text using ASCII box-drawing characters.
/// 
/// Substitutions:
/// r - y - 7    ╔ ═ ╦ ═ ╗
/// |   |   |    ║   ║   ║
/// p - + - d    ╠ ═ ╬ ═ ╣
/// |   |   |    ║   ║   ║
/// l - t - j    ╚ ═ ╩ ═ ╝      
///
/// Putting a backslash before a character (e.g. "\\r" or "\\\") will escape it.
pub fn ascii_box_chars<T: IntoIterator<Item = char>>(s: T, x: u32, mut y: u32) {
    let mut escaped = false;
    let mut nx = x;
    for c in s {
        if escaped {
            pixel(c, x, y);
            nx += 1;
            escaped = false;
            continue
        }

        match c {
            '\\' => escaped = true,
            '\n' => {
                nx = x;
                y += 1;
            }

            'r' => pixel(BoxDrawingChar::TopLeftCorner.into(), nx, y),
            '-' => pixel(BoxDrawingChar::Horizontal.into(), nx, y),
            'y' => pixel(BoxDrawingChar::TopT.into(), nx, y),
            '7' => pixel(BoxDrawingChar::TopRightCorner.into(), nx, y),
            '|' => pixel(BoxDrawingChar::Vertical.into(), nx, y),
            'p' => pixel(BoxDrawingChar::LeftT.into(), nx, y),
            '+' => pixel(BoxDrawingChar::MiddleT.into(), nx, y),
            'd' => pixel(BoxDrawingChar::RightT.into(), nx, y),
            'l' => pixel(BoxDrawingChar::BottomLeftCorner.into(), nx, y),
            't' => pixel(BoxDrawingChar::BottomT.into(), nx, y),
            'j' => pixel(BoxDrawingChar::BottomRightCorner.into(), nx, y),

            _ => pixel(c, nx, y),
        }
        nx += 1;
    }
}

/// Draw a box using ASCII box-drawing characters.
pub fn ascii_box(x1: u32, y1: u32, x2: u32, y2: u32) -> Result<(), &'static str> {
    orth_line(BoxDrawingChar::Horizontal.into(), x1 + 1, y1, x2 - 1, y1)?;
    orth_line(BoxDrawingChar::Horizontal.into(), x1 + 1, y2, x2 - 1, y2)?;
    orth_line(BoxDrawingChar::Vertical.into(), x1, y1 + 1, x1, y2 - 1)?;
    orth_line(BoxDrawingChar::Vertical.into(), x2, y1 + 1, x2, y2 - 1)?;

    pixel(BoxDrawingChar::TopLeftCorner.into(), x1, y1);
    pixel(BoxDrawingChar::TopRightCorner.into(), x2, y1);
    pixel(BoxDrawingChar::BottomLeftCorner.into(), x1, y2);
    pixel(BoxDrawingChar::BottomRightCorner.into(), x2, y2);

    Ok(())
}

/// Draw a filled rectangle onto the screen.
pub fn rect_fill(c: char, x1: u32, y1: u32, x2: u32, y2: u32) -> Result<(), &'static str> {
    let mut y = y1;
    while y != y2 {
        orth_line(c, x1, y, x2, y)?;
        y += 1;
    }

    Ok(())
}

/// Draw a triangle onto the screen.
pub fn triangle(
    c: char,
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
    x3: u32,
    y3: u32,
) -> Result<(), &'static str> {
    line(c, x1, y1, x2, y2)?;
    line(c, x1, y1, x3, y3)?;
    line(c, x3, y3, x2, y2)
}

/// Draw a filled triangle onto the screen.
pub fn triangle_fill(_c: char, _x1: u32, _y1: u32, _x2: u32, _y2: u32, _x3: u32, _y3: u32) {
    todo!();
}

/// Draw text onto the screen (non-wrapping, but respects linebreaks).
pub fn text<T: IntoIterator<Item = char>>(s: T, x: u32, mut y: u32) {
    let mut nx = x;
    for c in s {
        if c == '\n' {
            nx = x;
            y += 1;
        }

        pixel(c, nx, y);
        nx += 1;
    }
}

/// Set cursor to position.
pub fn goto(x: u32, y: u32) {
    escape(format!("{};{}H", y + 1, x + 1));
}

/// Put cursor to top of screen.
pub fn home() {
    goto(1, 1);
}

/// Put cursor to the bottom of the screen.
pub fn bot() {
    goto(0, 9998);
}

/// Flush everything you've drawn to stdout.
pub fn flush() {
    stdout().flush().unwrap();
}

/// A single-line textbox.
#[cfg(feature = "input")]
pub struct InputField {
    length: Option<usize>,
    data: String,
    pos: usize,

    pub up_arrow: bool,
    pub down_arrow: bool,
    pub newline: bool,
}

#[cfg(feature = "input")]
impl InputField {
    /// Creates a new textbox with a given (or no) length.
    pub fn new(length: Option<usize>) -> Self {
        Self {
            length,
            data: String::new(),
            pos: 0,

            up_arrow: false,
            down_arrow: false,
            newline: false,
        }
    }

    /// Get the contents of the textbox.
    pub fn get(&self) -> String {
        self.data.clone()
    }

    /// Set the contents of the textbox.
    /// Cursor is automatically set to the end.
    ///
    /// Returns false if the string is too large for the input box.
    pub fn set(&mut self, new: String) -> bool {
        if self.length.is_some() && new.len() > self.length.unwrap() {
            return false;
        }

        self.data = new;
        self.pos = self.data.len();

        true
    }

    /// Clears the contents of the textbox.
    pub fn clear(&mut self) {
        self.data.clear();
        self.pos = 0;
    }

    /// Draw the contents of the textbox.
    pub fn draw(&self, mut x: u32, y: u32) {
        decolor();
        for (i, ch) in self.data.chars().enumerate() {
            if i == self.pos {
                color_bg(7);
                color_fg(8);
            } else if i == self.pos + 1 {
                decolor();
            }

            pixel(ch, x, y);

            x += 1;
        }

        if self.pos == self.data.len() {
            color_bg(7);
            color_fg(8);
            pixel(' ', x, y);
        }
    }

    /// Collects input until it recieves a newline.
    ///
    /// NOTE: WILL overwrite any characters drawn in its path.
    /// NOTE: WILL overwrite one character past the end (it has to do with the cursor, it's unfixable :/).
    pub fn get_line(&mut self, x: u32, y: u32) {
        let stdout = Term::buffered_stdout();
        let _ = stdout.hide_cursor();
        loop {
            decolor();
            for i in 0..=self.data.len() as u32 {
                pixel(' ', x + i, y);
            }

            text(self.data.chars(), x, y);
            bot(); // for when the terminal doesn't hide the cursor, it looks slightly better
            flush();

            if let Ok(key) = stdout.read_key() {
                match key {
                    Key::Backspace => {
                        if self.pos > 0 {
                            self.pos -= 1;
                            self.data.remove(self.pos);
                        }
                    }
                    Key::Char(ch) => {
                        if self.data.len() < self.length.unwrap_or(usize::MAX) {
                            self.data.insert(self.pos, ch);
                            self.pos += 1;
                        }
                    }
                    Key::ArrowLeft => {
                        if self.pos > 0 {
                            self.pos -= 1;
                        }
                    }
                    Key::ArrowRight => {
                        if self.pos < self.data.len() {
                            self.pos += 1;
                        }
                    }
                    Key::Enter => break,
                    _ => {}
                }
            }
        }
        let _ = stdout.show_cursor();
    }

    /// Add a character to the textbox; if the textbox is full, returns false.
    pub fn ch(&mut self, c: char) -> bool {
        if self.data.len() >= self.length.unwrap_or(usize::MAX) {
            return false;
        }

        self.data.push(c);
        self.pos += 1;

        true
    }

    /// Tries to read a single key; non-blocking.
    pub fn poll(&mut self) {
        let stdout = Term::buffered_stdout();

        if let Ok(key) = stdout.read_key() {
            if key != Key::ArrowUp {
                self.up_arrow = false;
            } else {
                self.up_arrow = true;
            }

            if key != Key::ArrowDown {
                self.down_arrow = false;
            } else {
                self.down_arrow = true;
            }

            match key {
                Key::Backspace => {
                    if self.pos > 0 {
                        self.pos -= 1;
                        self.data.remove(self.pos);
                    }
                }
                Key::Char(ch) => {
                    if self.data.len() < self.length.unwrap_or(usize::MAX) {
                        self.data.insert(self.pos, ch);
                        self.pos += 1;
                    }
                }
                Key::ArrowLeft => {
                    if self.pos > 0 {
                        self.pos -= 1;
                    }
                }
                Key::ArrowRight => {
                    if self.pos < self.data.len() {
                        self.pos += 1;
                    }
                }
                Key::Enter => {
                    self.newline = true;
                }
                _ => {}
            }
        }
    }
}

#[cfg(feature = "input")]
pub struct InputManager {
    input: Term,
}

#[cfg(feature = "input")]
impl InputManager {
    pub fn new() -> Self {
        Self {
            input: Term::buffered_stdout(),
        }
    }

    pub fn poll(&self) -> Option<Key> {
        self.input.read_key().ok()
    }

    // TODO: doesn't work
    // pub fn keys(&self) -> HashSet<Key> {
    //     let mut keys = HashSet::new();
    //     while let Some(key) = self.poll() {
    //         keys.insert(key);
    //     }

    //     keys
    // }
}
