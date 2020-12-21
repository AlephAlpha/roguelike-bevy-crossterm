//! Bresenham's algorithms for drawing lines.
//!
//! Based on http://members.chello.at/~easyfilter/bresenham.html

use crate::{geometry::Quadrant, Point};

/// Line-drawing iterator
pub struct BresenhamLine {
    point: Point,
    end: Point,
    dx: i16,
    dy: i16,
    sx: i16,
    sy: i16,
    err: i16,
}

impl BresenhamLine {
    /// Creates a new iterator. Yields intermediate points between `start`
    /// and `end`. Does include `end` but not `start`.
    #[inline]
    pub fn new(start: Point, end: Point) -> BresenhamLine {
        let (quadrant, (dx, dy)) = Quadrant::from_points(start, end);

        let (sx, sy) = quadrant.from_quadrant0((0, 0), (1, 1));

        BresenhamLine {
            point: start,
            end,
            dx,
            dy,
            sx,
            sy,
            err: dx - dy,
        }
    }
}

impl Iterator for BresenhamLine {
    type Item = Point;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.point == self.end {
            return None;
        }

        let e2 = 2 * self.err;
        if e2 >= -self.dy {
            self.point.0 += self.sx;
            self.err -= self.dy;
        }
        if e2 <= self.dx {
            self.point.1 += self.sy;
            self.err += self.dx;
        }

        Some(self.point)
    }
}
