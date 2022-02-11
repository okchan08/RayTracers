use crate::base::vec::Vec3;
use crate::object::ray::Ray;

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
