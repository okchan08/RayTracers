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
    let a = ray.direction().norm() * ray.direction().norm();
    let half_b = v_oc.dot(&ray.direction());
    //let b = ray.direction().dot(&v_oc) * 2.0;
    let c = v_oc.norm() * v_oc.norm() - self.radius * self.radius;
    let d = half_b * half_b - a * c;

    if d > 0.0 {
      let root = d.sqrt();
      let mut temp = (-half_b - root) / a;
      if temp < lower_range || temp > upper_range {
        temp = (-half_b + root) / a;
        if temp < lower_range || temp > upper_range {
          return None;
        }
      }
      let pos = ray.at(temp);
      let mut hit_info = HitInfo::new(
        temp,
        pos.clone(),
        (pos.clone() - self.center()) / self.radius,
        self.material,
        &ray,
      );
      hit_info.set_front_face(&ray, &((pos.clone() - self.center()) / self.radius));
      return Some(hit_info);
    }
    None
  }

  fn scatter(&self, incoming_ray: &Ray, hit_info: &HitInfo) -> Option<(Ray, Vec3)> {
    match hit_info.get_hit_material() {
      Material::Lambertian { albedo } => {
        //let mut scatter_dir =
        //  hit_info.get_normal().clone() + Vec3::gen_random_vector_in_unit_shpere().normalize();
        let mut scatter_dir = Vec3::gen_random_vector_in_unit_shpere().normalize();
        if scatter_dir.near_zero() {
          scatter_dir = hit_info.get_normal().clone();
        }

        let scattered = Ray::new(hit_info.get_poisition().clone(), scatter_dir);
        return Some((scattered, albedo.clone()));
      }
      Material::Metal { albedo, fuzzy } => {
        let reflected = Vec3::reflect(
          &incoming_ray.direction().normalize(),
          &hit_info.get_normal(),
        ) + Vec3::gen_random_vector_in_unit_shpere() * fuzzy.clamp(0.0, 1.0);
        let scatterd = Ray::new(hit_info.get_poisition().clone(), reflected);
        if scatterd.direction().dot(hit_info.get_normal()) > 0.0 {
          return Some((scatterd, albedo.clone()));
        }
        None
      }
      Material::Dielectric { refraction_index } => {
        let attenuation = Vec3::from_one(1.0);
        let refraction_ratio = if hit_info.front_face() {
          1.0 / refraction_index
        } else {
          *refraction_index
        };

        let unit_dir = incoming_ray.direction().normalize();
        let refracted = Vec3::refract(&unit_dir, &hit_info.get_normal(), refraction_ratio);
        let scatterd = Ray::new(hit_info.get_poisition().clone(), refracted);
        return Some((scatterd, attenuation));
      }
      _ => Some((
        Ray::new(Vec3::zero_vector(), Vec3::zero_vector()),
        Vec3::zero_vector(),
      )),
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
