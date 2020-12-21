//! Field of view algorithms.
//!
//! Based on http://www.adammil.net/blog/v125_Roguelike_Vision_Algorithms.html.

mod ray_casting;
mod shadow_casting;

use crate::Point;
pub use ray_casting::ray_casting_fov;
pub use shadow_casting::shadow_casting_fov;

pub trait Map2D {
    fn is_opaque(&self, point: Point) -> bool;
    fn is_in_bound(&self, point: Point) -> bool;
}
