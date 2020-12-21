//! Bresenham's algorithms for drawing circles.
//!
//! Based on http://members.chello.at/~easyfilter/bresenham.html

use crate::{
    geometry::{Octant, Quadrant},
    Point,
};

/// Circle-drawing iterator
pub struct BresenhamCircle {
    quadrant: Quadrant,
    point: Point,
    center: Point,
    r: i16,
    err: i16,
}

impl BresenhamCircle {
    #[inline]
    pub fn new(center: Point, r: i16) -> BresenhamCircle {
        BresenhamCircle {
            quadrant: Quadrant(0),
            point: (0, r),
            center,
            r,
            err: 2 - 2 * r,
        }
    }
}

impl Iterator for BresenhamCircle {
    type Item = Point;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.point.1 == 0 {
            if self.quadrant.0 == 3 {
                return None;
            } else {
                self.quadrant.0 += 1;
                self.point = (0, self.r);
                self.err = 2 - 2 * self.r;
            }
        }

        let e = self.err;
        if e <= self.point.0 {
            self.point.0 += 1;
            self.err += self.point.0 * 2 + 1;
        }
        if e > -self.point.1 {
            self.point.1 -= 1;
            self.err += -self.point.1 * 2 + 1;
        }

        let point = self
            .quadrant
            .from_quadrant0_rotating(self.center, self.point);

        Some(point)
    }
}

/// Circle-drawing iterator
pub struct BresenhamCircleNoDiag {
    octant: Octant,
    point: Point,
    center: Point,
    r: i16,
    err: i16,
}

impl BresenhamCircleNoDiag {
    #[inline]
    pub fn new(center: Point, r: i16) -> BresenhamCircleNoDiag {
        BresenhamCircleNoDiag {
            octant: Octant(0),
            point: (0, r),
            center,
            r,
            err: 2 - 2 * r,
        }
    }
}

impl Iterator for BresenhamCircleNoDiag {
    type Item = Point;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let point = self.octant.from_octant0(self.center, self.point);

        if self.point.0 == self.point.1 {
            if self.octant.0 == 7 {
                return None;
            } else {
                self.octant.0 += 1;
                self.point = (0, self.r);
                self.err = 2 - 2 * self.r;
                return Some(point);
            }
        }

        if self.err > 0 {
            self.point.1 -= 1;
            self.err += -self.point.1 * 2 + 1;
        } else {
            self.point.0 += 1;
            self.err += self.point.0 * 2 + 1;
        }

        Some(point)
    }
}
