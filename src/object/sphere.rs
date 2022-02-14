use crate::base::vec::Vec3;
use crate::object::hit::HitInfo;
use crate::object::ray::Ray;
use crate::object::shape::Shape;

pub struct Sphere {
  center: Vec3,
  radius: f64,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f64) -> Self {
    Self {
      center: center,
      radius: radius,
    }
  }
  /// 球体と光線の衝突判定を2次方程式の解の公式を用いて行う
  pub fn hit_sphere(&self, ray: &Ray) -> f64 {
    let v_oc = ray.origin() - &self.center;
    let a = ray.direction().dot(ray.direction());
    let b = ray.direction().dot(&v_oc) * 2.0;
    let c = v_oc.dot(&v_oc) - self.radius * self.radius;
    let d = b * b - 4.0 * a * c;

    if d < 0.0 {
      -1.0
    } else {
      (-b - d.sqrt()) / (2.0 * a)
    }
  }

  pub fn center(&self) -> Vec3 {
    self.center
  }
}

impl Shape for Sphere {
  fn hit(sphere: &Self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
    let v_oc = ray.origin() - &sphere.center;
    let a = ray.direction().dot(ray.direction());
    let b = ray.direction().dot(&v_oc) * 2.0;
    let c = v_oc.dot(&v_oc) - sphere.radius * sphere.radius;
    let d = b * b - 4.0 * a * c;

    if d > 0.0 {
      let root = d.sqrt();
      let mut temp = (-b - root) / (2.0 * a);
      if temp < t1 && temp > t0 {
        let pos = ray.at(temp);
        Some(HitInfo::new(
          temp,
          pos.clone(),
          (pos - sphere.center()).dir(1.0 / sphere.radius),
        ));
      }
      temp = (-b + root) / (2.0 * a);
      if temp < t1 && temp > t0 {
        let pos = ray.at(temp);
        Some(HitInfo::new(
          temp,
          pos.clone(),
          (pos - sphere.center()).dir(1.0 / sphere.radius),
        ));
      }
    }
    None
  }
}

#[cfg(test)]
mod sphere_test {
  use super::*;
  #[test]
  fn test_hit() {
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
    let r1 = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    let r2 = Ray::new(Vec3::new(3.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));

    assert_eq!(sphere.hit_sphere(&r1), true);
    assert_eq!(sphere.hit_sphere(&r2), false);
  }
}
