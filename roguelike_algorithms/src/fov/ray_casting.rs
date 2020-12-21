use crate::{
    fov::Map2D,
    geometry::{BresenhamCircleNoDiag, BresenhamLine},
    Point,
};

fn scan_fov_line<'a, MAP>(
    map: &'a MAP,
    start: Point,
    end: Point,
) -> impl Iterator<Item = Point> + 'a
where
    MAP: Map2D + ?Sized,
{
    let mut visible = true;
    BresenhamLine::new(start, end).take_while(move |&point| {
        map.is_in_bound(point) && visible && {
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

    for point in BresenhamCircleNoDiag::new(start, range) {
        scan_fov_line(map, start, point).for_each(&mut callback);
    }
}
