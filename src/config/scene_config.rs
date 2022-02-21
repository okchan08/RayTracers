use serde::{Deserialize, Serialize};

use crate::base::math::{get_random_in_range, get_uniform_random};
use crate::base::vec::Vec3;
use crate::object::material::Material;
use crate::object::shape::Shape;
use crate::object::sphere::Sphere;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SceneConfig {
  pub objects: Vec<ObjectConfig>,
  #[serde(rename(deserialize = "camera"))]
  pub camera_config: Option<CameraConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename(deserialize = "camera"))]
pub struct CameraConfig {
  pub lookfrom: (f64, f64, f64),
  pub lookat: (f64, f64, f64),
  pub distance_to_focus: f64,
  pub vup: (f64, f64, f64),
  pub vofv: f64,
  pub aspect: f64,
  pub aperture: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ObjectConfig {
  #[serde(rename(serialize = "sphere", deserialize = "sphere"))]
  Sphere {
    center: (f64, f64, f64),
    radius: f64,
    name: String,
    material: MaterialConfig,
  },
  #[serde(rename(serialize = "box", deserialize = "box"))]
  Box {
    left: (f64, f64, f64),
    right: (f64, f64, f64),
    name: String,
    material: MaterialConfig,
  },
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum MaterialConfig {
  #[serde(rename(serialize = "lambertian", deserialize = "lambertian"))]
  Lambertian { albedo: (f64, f64, f64) },
  #[serde(rename(serialize = "metal", deserialize = "metal"))]
  Metal { albedo: (f64, f64, f64), fuzzy: f64 },
  #[serde(rename(serialize = "dielectric", deserialize = "dielectric"))]
  Dielectric { refraction_index: f64 },
}

impl CameraConfig {
  pub fn default() -> Self {
    println!("here");
    Self {
      lookfrom: (0.0, -20.0, 3.0),
      lookat: (0.0, -1.0, 0.0),
      vup: (0.0, 1.0, -0.5),
      vofv: 20.0,
      aspect: 680.0 / 460.0,
      aperture: 0.1,
      distance_to_focus: 19.10, // distance between lookfrom and lookat
    }
  }
}

impl ObjectConfig {
  pub fn to_object(&self) -> Box<dyn Shape> {
    match self {
      ObjectConfig::Sphere {
        center,
        radius,
        name,
        material,
      } => Box::new(Sphere::new(
        Vec3::from_tuple(*center),
        *radius,
        name.to_string(),
        material.to_material(),
      )),
      _ => {
        unimplemented!()
      }
    }
  }

  pub fn gen_random() -> Vec<ObjectConfig> {
    let mut ret = vec![];
    for a in -11..11 {
      for b in -11..11 {
        let choose_material = get_uniform_random();
        let radius = 0.3;
        let center = Vec3::new(
          (a as f64) * 1.5 + 0.9 * get_uniform_random(),
          (b as f64) * 1.5 + 0.9 * get_uniform_random(),
          radius / 2.0,
        );
        let name = format!("sphere {}{}", a, b);
        if (center - Vec3::new(4.0, 0.0, radius / 2.0)).norm() > 0.9 {
          if choose_material < 0.33 {
            let albedo = Vec3::new(
              get_random_in_range(0.0, 0.99),
              get_random_in_range(0.0, 0.99),
              get_random_in_range(0.0, 0.99),
            );
            ret.push(ObjectConfig::Sphere {
              center: center.to_tuple(),
              radius: radius,
              name: name,
              material: MaterialConfig::Lambertian {
                albedo: albedo.to_tuple(),
              },
            });
          } else if choose_material < 0.666 {
            let albedo = Vec3::new(
              get_random_in_range(0.5, 0.99),
              get_random_in_range(0.5, 0.99),
              get_random_in_range(0.5, 0.99),
            );
            let fuzzy = get_random_in_range(0.0, 0.5);
            ret.push(ObjectConfig::Sphere {
              center: center.to_tuple(),
              radius: radius,
              name: name,
              material: MaterialConfig::Metal {
                albedo: albedo.to_tuple(),
                fuzzy: fuzzy,
              },
            });
          } else {
            ret.push(ObjectConfig::Sphere {
              center: center.to_tuple(),
              radius: radius,
              name: name,
              material: MaterialConfig::Dielectric {
                refraction_index: 1.5,
              },
            });
          }
        }
      }
    }
    ret
  }
}

impl MaterialConfig {
  pub fn to_material(&self) -> Material {
    match self {
      MaterialConfig::Lambertian { albedo } => Material::Lambertian {
        albedo: Vec3::new(albedo.0, albedo.1, albedo.2),
      },
      MaterialConfig::Metal { albedo, fuzzy } => Material::Metal {
        albedo: Vec3::new(albedo.0, albedo.1, albedo.2),
        fuzzy: *fuzzy,
      },
      MaterialConfig::Dielectric { refraction_index } => Material::Dielectric {
        refraction_index: *refraction_index,
      },
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use serde_yaml;
  #[test]
  fn test_object_gen_random() {
    let objects = ObjectConfig::gen_random();
    let str = serde_yaml::to_string(&objects);
    assert_eq!(objects.len() > 0, true);
    assert_eq!(str.is_ok(), true);
    println!("{}", str.unwrap());
  }
}
