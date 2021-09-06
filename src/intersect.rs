use crate::ray::Ray;
use crate::hit::Hit;

pub trait Intersect {
    fn intersect(ray: Ray) -> Option<Hit>;
}