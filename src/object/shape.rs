use crate::base::vec::Vec3;
use crate::object::hit::HitInfo;
use crate::object::ray::Ray;

pub trait Shape: Sync {
  fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo>;

  fn scatter(&self, incoming_ray: &Ray, hit_info: &HitInfo) -> Option<(Ray, Vec3)>;

  fn name(&self) -> &str;
}
