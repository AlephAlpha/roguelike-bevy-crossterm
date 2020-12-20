//! Field of view algorithms.
//!
//! Based on http://www.adammil.net/blog/v125_Roguelike_Vision_Algorithms.html.

use crate::{geometry::Bresenham, Octant, Point};

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
    MAP: Map2D + ?Sized,
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
    MAP: Map2D + ?Sized,
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

/// Represents the slope Y/X as a rational number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Slope {
    pub x: i16,
    pub y: i16,
}

#[allow(clippy::clippy::too_many_arguments)]
fn shadow_casting_octant<MAP, F>(
    map: &MAP,
    octant: Octant,
    start: Point,
    range: i16,
    x: i16,
    mut top: Slope,
    mut bottom: Slope,
    callback: &mut F,
) where
    MAP: Map2D + ?Sized,
    F: FnMut(Point),
{
    for x in x..=range {
        let top_y = if top.x == 1 {
            x
        } else {
            ((x * 2 + 1) * top.y + top.x - 1) / (top.x * 2)
        };
        let bottom_y = if bottom.y == 0 {
            0
        } else {
            ((x * 2 - 1) * bottom.y + bottom.x) / (bottom.x * 2)
        };

        let mut was_opaque: Option<bool> = None;

        for y in bottom_y..=top_y {
            let (dx, dy) = octant.from_octant0((x, y));
            let point = (start.0 + dx, start.1 + dy);

            let in_range = map.is_in_bound(point) && map.distance(start, point) <= range;

            if in_range {
                callback(point);
            }

            let is_opaque = !in_range || map.is_opaque(point);

            if x != range {
                if is_opaque {
                    if was_opaque == Some(false) {
                        let new_top = Slope {
                            x: x * 2 + 1,
                            y: y * 2 - 1,
                        };
                        if !in_range || y == top_y {
                            top = new_top;
                            break;
                        } else {
                            shadow_casting_octant(
                                map,
                                octant,
                                start,
                                range,
                                x + 1,
                                new_top,
                                bottom,
                                callback,
                            );
                        }
                    }
                    was_opaque = Some(true);
                } else {
                    if was_opaque == Some(true) {
                        bottom = Slope {
                            x: x * 2 - 1,
                            y: y * 2 - 1,
                        };
                    }
                    was_opaque = Some(false);
                }
            }
        }

        if was_opaque != Some(false) {
            break;
        }
    }
}

pub fn shadow_casting_fov<MAP, F>(map: &MAP, start: Point, range: i16, mut callback: F)
where
    MAP: Map2D + ?Sized,
    F: FnMut(Point),
{
    callback(start);

    for octant in 0..8 {
        let octant = Octant(octant);
        shadow_casting_octant(
            map,
            octant,
            start,
            range,
            1,
            Slope { x: 1, y: 1 },
            Slope { x: 1, y: 0 },
            &mut callback,
        );
    }
}
