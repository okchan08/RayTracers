use serde::{Deserialize, Serialize};

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
  #[serde(rename(deserialize = "sphere"))]
  Sphere {
    center: (f64, f64, f64),
    radius: f64,
    name: String,
    material: MaterialConfig,
  },
  #[serde(rename(deserialize = "box"))]
  Box {
    left: (f64, f64, f64),
    right: (f64, f64, f64),
    name: String,
    material: MaterialConfig,
  },
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum MaterialConfig {
  #[serde(rename(deserialize = "lambertian"))]
  Lambertian { albedo: (f64, f64, f64) },
  #[serde(rename(deserialize = "metal"))]
  Metal { albedo: (f64, f64, f64), fuzzy: f64 },
  #[serde(rename(deserialize = "dielectric"))]
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
