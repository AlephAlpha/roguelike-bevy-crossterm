use crate::{fov::Map2D, geometry::Octant, Point};

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
        }
        .min(((range * range - x * x) as f32).sqrt() as i16);
        let bottom_y = if bottom.y == 0 {
            0
        } else {
            ((x * 2 - 1) * bottom.y + bottom.x) / (bottom.x * 2)
        };

        let mut was_opaque: Option<bool> = None;

        for y in bottom_y..=top_y {
            let point = octant.from_octant0(start, (x, y));

            let in_range = map.is_in_bound(point);

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

    for octant in Octant::all() {
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
