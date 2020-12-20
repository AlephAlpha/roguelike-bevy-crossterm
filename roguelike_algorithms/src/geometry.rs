//! Bresenham's line drawing algorithm.
//!
//! Modified from https://github.com/mbr/bresenham-rs

use super::{Octant, Point};

/// Line-drawing iterator
pub struct Bresenham {
    x: i16,
    y: i16,
    x1: i16,
    error_x: i16,
    error_y: i16,
    error: i16,
    octant: Octant,
}

impl Bresenham {
    /// Creates a new iterator.Yields intermediate points between `start`
    /// and `end`. Does include `start` but not `end`.
    #[inline]
    pub fn new(start: Point, end: Point) -> Bresenham {
        let octant = Octant::from_points(start, end);

        let start = octant.to_octant0(start);
        let end = octant.to_octant0(end);

        let dx = end.0 - start.0;
        let dy = end.1 - start.1;

        Bresenham {
            x: start.0,
            y: start.1,
            x1: end.0,
            error_x: dx * 2,
            error_y: dy * 2,
            error: -dx,
            octant,
        }
    }
}

impl Iterator for Bresenham {
    type Item = Point;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.x1 {
            return None;
        }

        self.x += 1;

        self.error += self.error_y;

        if self.error > 0 {
            self.y += 1;
            self.error -= self.error_x;
        }

        let p = (self.x, self.y);

        Some(self.octant.from_octant0(p))
    }
}
