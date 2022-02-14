use crate::object::hit::HitInfo;
use crate::object::ray::Ray;

pub trait Shape {
  fn hit(object: &Self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo>;
}
