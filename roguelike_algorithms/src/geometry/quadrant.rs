pub use crate::Point;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Quadrant(pub u8);

/// adapted from http://codereview.stackexchange.com/a/95551
impl Quadrant {
    #[inline]
    pub fn from_points(start: Point, end: Point) -> (Quadrant, Point) {
        let mut dx = end.0 - start.0;
        let mut dy = end.1 - start.1;

        let mut quadrant = 0;

        if dy < 0 {
            dx = -dx;
            dy = -dy;
            quadrant += 2;
        }

        if dx < 0 {
            dx = -dx;
            quadrant += 1;
        }

        (Quadrant(quadrant), (dx, dy))
    }

    #[inline]
    pub fn from_points_rotating(start: Point, end: Point) -> (Quadrant, Point) {
        let mut dx = end.0 - start.0;
        let mut dy = end.1 - start.1;

        let mut quadrant = 0;

        if dy < 0 {
            dx = -dx;
            dy = -dy;
            quadrant += 2;
        }

        if dx < 0 {
            std::mem::swap(&mut dx, &mut dy);
            dy = -dy;
            quadrant += 1;
        }

        (Quadrant(quadrant), (dx, dy))
    }

    #[inline]
    pub fn to_quadrant0(&self, p: Point) -> Point {
        match self.0 {
            0 => (p.0, p.1),
            1 => (-p.0, p.1),
            2 => (-p.0, -p.1),
            3 => (p.0, -p.1),
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn to_quadrant0_rotating(&self, p: Point) -> Point {
        match self.0 {
            0 => (p.0, p.1),
            1 => (p.1, -p.0),
            2 => (-p.0, -p.1),
            3 => (-p.1, p.0),
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn from_quadrant0(&self, start: Point, d: Point) -> Point {
        let (dx, dy) = match self.0 {
            0 => (d.0, d.1),
            1 => (-d.0, d.1),
            2 => (-d.0, -d.1),
            3 => (d.0, -d.1),
            _ => unreachable!(),
        };
        (start.0 + dx, start.1 + dy)
    }

    #[inline]
    pub fn from_quadrant0_rotating(&self, start: Point, d: Point) -> Point {
        let (dx, dy) = match self.0 {
            0 => (d.0, d.1),
            1 => (-d.1, d.0),
            2 => (-d.0, -d.1),
            3 => (d.1, -d.0),
            _ => unreachable!(),
        };
        (start.0 + dx, start.1 + dy)
    }

    #[inline]
    /// A iterator over all quadrants.
    pub fn all() -> impl Iterator<Item = Quadrant> {
        (0..2).map(Quadrant)
    }
}
