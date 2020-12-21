use crate::Point;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Octant(pub u8);

/// adapted from http://codereview.stackexchange.com/a/95551
impl Octant {
    #[inline]
    pub fn from_points(start: Point, end: Point) -> (Octant, Point) {
        let mut dx = end.0 - start.0;
        let mut dy = end.1 - start.1;

        let mut octant = 0;

        if dy < 0 {
            dx = -dx;
            dy = -dy;
            octant += 4;
        }

        if dx < 0 {
            std::mem::swap(&mut dx, &mut dy);
            dy = -dy;
            octant += 2;
        }

        if dx < dy {
            std::mem::swap(&mut dx, &mut dy);
            octant += 1;
        }

        (Octant(octant), (dx, dy))
    }

    #[inline]
    pub fn to_octant0(&self, p: Point) -> Point {
        match self.0 {
            0 => (p.0, p.1),
            1 => (p.1, p.0),
            2 => (p.1, -p.0),
            3 => (-p.0, p.1),
            4 => (-p.0, -p.1),
            5 => (-p.1, -p.0),
            6 => (-p.1, p.0),
            7 => (p.0, -p.1),
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn from_octant0(&self, start: Point, d: Point) -> Point {
        let (dx, dy) = match self.0 {
            0 => (d.0, d.1),
            1 => (d.1, d.0),
            2 => (-d.1, d.0),
            3 => (-d.0, d.1),
            4 => (-d.0, -d.1),
            5 => (-d.1, -d.0),
            6 => (d.1, -d.0),
            7 => (d.0, -d.1),
            _ => unreachable!(),
        };
        (start.0 + dx, start.1 + dy)
    }

    #[inline]
    /// A iterator over all octants.
    pub fn all() -> impl Iterator<Item = Octant> {
        (0..8).map(Octant)
    }
}
