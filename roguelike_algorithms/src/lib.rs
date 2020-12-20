pub mod fov;
pub mod geometry;

pub type Point = (i16, i16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Octant(u8);

impl Octant {
    /// adapted from http://codereview.stackexchange.com/a/95551
    #[inline]
    pub fn from_points(start: Point, end: Point) -> Octant {
        let mut dx = end.0 - start.0;
        let mut dy = end.1 - start.1;

        let mut octant = 0;

        if dy < 0 {
            dx = -dx;
            dy = -dy;
            octant += 4;
        }

        if dx < 0 {
            let tmp = dx;
            dx = dy;
            dy = -tmp;
            octant += 2
        }

        if dx < dy {
            octant += 1
        }

        Octant(octant)
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
    pub fn from_octant0(&self, p: Point) -> Point {
        match self.0 {
            0 => (p.0, p.1),
            1 => (p.1, p.0),
            2 => (-p.1, p.0),
            3 => (-p.0, p.1),
            4 => (-p.0, -p.1),
            5 => (-p.1, -p.0),
            6 => (p.1, -p.0),
            7 => (p.0, -p.1),
            _ => unreachable!(),
        }
    }
}
