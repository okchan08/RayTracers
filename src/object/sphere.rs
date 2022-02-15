use crate::base::vec::Vec3;
use crate::object::hit::HitInfo;
use crate::object::material::Material;
use crate::object::ray::Ray;
use crate::object::shape::Shape;

pub struct Sphere {
  center: Vec3,
  radius: f64,
  name: String,
  material: Material,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f64, name: String, material: Material) -> Self {
    Self {
      center: center,
      radius: radius,
      name: name,
      material: material,
    }
  }

  pub fn center(&self) -> Vec3 {
    self.center
  }
}

impl Shape for Sphere {
  fn hit(&self, ray: &Ray, lower_range: f64, upper_range: f64) -> Option<HitInfo> {
    let v_oc = ray.origin() - &self.center();
    let a = ray.direction().dot(ray.direction());
    let b = ray.direction().dot(&v_oc) * 2.0;
    let c = v_oc.dot(&v_oc) - self.radius * self.radius;
    let d = b * b - 4.0 * a * c;

    if d > 0.0 {
      let root = d.sqrt();
      let mut temp = (-b - root) / (2.0 * a);
      if temp < upper_range && temp > lower_range {
        let pos = ray.at(temp);
        return Some(HitInfo::new(
          temp,
          pos.clone(),
          (pos - self.center()).dir(1.0 / self.radius),
          self.material,
        ));
      }
      temp = (-b + root) / (2.0 * a);
      if temp < upper_range && temp > lower_range {
        let pos = ray.at(temp);
        return Some(HitInfo::new(
          temp,
          pos.clone(),
          (pos - self.center()).dir(1.0 / self.radius),
          self.material,
        ));
      }
    }
    None
  }

  fn scatter(&self, hit_info: &HitInfo) -> (Ray, Vec3, bool) {
    match hit_info.get_hit_material() {
      Material::Lambertian { albedo } => {
        let target = hit_info.get_normal().clone()
          + hit_info.get_poisition().clone()
          + Vec3::gen_random_vector_in_unit_shpere();

        let scattered = Ray::new(
          hit_info.get_poisition().clone(),
          target - hit_info.get_poisition().clone(),
        );
        return (scattered, albedo.clone(), true);
      }
      _ => (
        Ray::new(Vec3::zero_vector(), Vec3::zero_vector()),
        Vec3::zero_vector(),
        false,
      ),
    }
  }

  fn name(&self) -> &str {
    self.name.as_str()
  }
}

#[cfg(test)]
mod sphere_test {
  use super::*;
  #[test]
  fn test_hit() {
    let sphere = Sphere::new(
      Vec3::new(0.0, 0.0, 0.0),
      1.0,
      "sample".to_string(),
      Material::Black,
    );
    let r1 = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    let r2 = Ray::new(Vec3::new(3.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));

    assert_eq!(sphere.hit(&r1, 0.0, std::f64::MAX).is_some(), true);
    assert_eq!(sphere.hit(&r2, 0.0, std::f64::MAX).is_none(), true);
  }
}
