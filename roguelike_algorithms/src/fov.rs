//! Field of view algorithms.
//!
//! Based on http://www.adammil.net/blog/v125_Roguelike_Vision_Algorithms.html.

use crate::{geometry::Bresenham, Point};

pub trait Map2D {
    fn is_opaque(&self, point: Point) -> bool;
    fn is_in_bound(&self, point: Point) -> bool;
    fn distance(&self, start: Point, end: Point) -> i16;
}

fn scan_fov_line<'a, MAP>(
    map: &'a MAP,
    start: Point,
    end: Point,
    range: i16,
) -> impl Iterator<Item = Point> + 'a
where
    MAP: Map2D,
{
    let mut visible = true;
    Bresenham::new(start, end).take_while(move |&point| {
        map.is_in_bound(point) && map.distance(start, point) <= range && visible && {
            if map.is_opaque(point) {
                visible = false;
            }
            true
        }
    })
}

pub fn ray_casting_fov<MAP, F>(map: &MAP, start: Point, range: i16, mut callback: F)
where
    MAP: Map2D,
    F: FnMut(Point),
{
    callback(start);

    let left = start.0 - range;
    let right = start.0 + range;
    let top = start.1 - range;
    let bottom = start.1 + range;

    for x in left..right {
        scan_fov_line(map, start, (x, top), range)
            .chain(scan_fov_line(map, start, (x, bottom), range))
            .for_each(&mut callback);
    }

    for y in top..bottom {
        scan_fov_line(map, start, (left, y), range)
            .chain(scan_fov_line(map, start, (right, y), range))
            .for_each(&mut callback);
    }
}
