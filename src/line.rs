//! The [`Iter`] struct, for generating points on a line.
#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct Iter {
    x: i64,
    y: i64,

    x1: i64,
    y1: i64,

    dx: i64,
    dy: i64,
    err: i64,

    xx: i64,
    xy: i64,
    yx: i64,
    yy: i64,
}

impl Iter {
    pub fn new(x1: u32, y1: u32, x2: u32, y2: u32) -> Self {
        let mut li = Self {
            x1: i64::from(x1),
            y1: i64::from(y1),
            dx: i64::from(x2) - i64::from(x1),
            dy: i64::from(y2) - i64::from(y1),
            ..Default::default()
        };

        let sx = if x2 > x1 { 1 } else { -1 };
        let sy = if y2 > y1 { 1 } else { -1 };

        li.err = (li.dy << 1) - li.dx;

        if li.dx > li.dy {
            li.xx = sx;
            li.xy = 0i64;
            li.yx = 0i64;
            li.yy = sy;
        } else {
            std::mem::swap(&mut li.dx, &mut li.dy);
            li.xx = 0i64;
            li.xy = sy;
            li.yx = sx;
            li.yy = 0i64;
        }

        li.x1 = i64::from(x1);
        li.y1 = i64::from(y1);

        li
    }
}

impl Iterator for Iter {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x > self.dx {
            return None;
        }

        let x = (self.x1 + self.x * self.xx + self.y * self.yx) as u32;
        let y = (self.y1 + self.x * self.xy + self.y * self.yy) as u32;

        if self.err >= 0 {
            self.y += 1;
            self.err -= self.dx << 1;
        }

        self.err += self.dy << 1;
        self.x += 1;

        Some((x, y))
    }
}
