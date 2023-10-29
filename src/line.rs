#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct LineIter {
    x: i32,
    y: i32,

    x1: i32,
    y1: i32,

    dx: i32,
    dy: i32,
    err: i32,

    xx: i32,
    xy: i32,
    yx: i32,
    yy: i32,
}

impl LineIter {
    pub fn new(x1: u32, y1: u32, x2: u32, y2: u32) -> Self {
        let mut li = Self {
            x1: x1 as i32,
            y1: y1 as i32,
            dx: x2 as i32 - x1 as i32,
            dy: y2 as i32 - y1 as i32,
            ..Default::default()
        };

        let sx = if x2 > x1 { 1 } else { -1 };
        let sy = if y2 > y1 { 1 } else { -1 };

        li.err = (li.dy << 1) - li.dx;

        if li.dx > li.dy {
            li.xx = sx;
            li.xy = 0i32;
            li.yx = 0i32;
            li.yy = sy;
        } else {
            std::mem::swap(&mut li.dx, &mut li.dy);
            li.xx = 0i32;
            li.xy = sy;
            li.yx = sx;
            li.yy = 0i32;
        }

        li.x1 = x1 as i32;
        li.y1 = y1 as i32;

        li
    }
}

impl Iterator for LineIter {
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
