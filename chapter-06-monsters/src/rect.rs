#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rect {
    pub x1: i16,
    pub x2: i16,
    pub y1: i16,
    pub y2: i16,
}

impl Rect {
    pub fn new(x: i16, y: i16, w: i16, h: i16) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    // Returns true if this overlaps with other
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> (i16, i16) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}
