use crate::base::color::{Color, WHITE};
use crate::base::vec::Vec3;
use crate::object::camera::Camera;
use crate::object::ray::Ray;
use crate::object::shape::Shape;

use rayon::prelude::*;

use std::borrow::Borrow;

pub struct BufferWrapper(Vec<u32>);

impl Borrow<[u8]> for BufferWrapper {
  fn borrow(&self) -> &[u8] {
    unsafe { std::slice::from_raw_parts(self.0.as_ptr() as *const u8, self.0.len() * 4) }
  }
}

pub struct Scene {
  camera: Camera,
  background_color: Color,
  width: u32,
  height: u32,
  super_samples: u32,
  max_scatter_depth: u32,
  objects: Vec<Box<dyn Shape>>,
}

impl Scene {
  pub fn new(
    camera: Camera,
    width: u32,
    height: u32,
    super_samples: u32,
    max_scatter_depth: u32,
  ) -> Self {
    Scene {
      camera: camera,
      background_color: WHITE,
      width: width,
      height: height,
      super_samples: super_samples,
      max_scatter_depth: max_scatter_depth,
      objects: vec![],
    }
  }

  pub fn set_camera(&mut self, camera: Camera) {
    self.camera = camera;
  }

  pub fn set_background_color(&mut self, color: Color) {
    self.background_color = color;
  }

  pub fn add_object(&mut self, object: Box<dyn Shape>) {
    self.objects.push(object);
  }

  pub fn render(&self) -> BufferWrapper {
    let mut buf = vec![0u32; (self.width * self.height) as usize];

    buf.par_iter_mut().enumerate().for_each(|(i, pixel)| {
      let y = ((i as u64) / (self.width as u64)) as f64;
      let x = (i as f64) - y * (self.width as f64);
      let u = (x as f64) / (self.width as f64);
      let v = (y as f64) / (self.height as f64);
      let mut col = Vec3::from_one(0.0);
      for _ in 0..self.super_samples {
        let ray = self.camera.get_ray(u, v);
        let c = self.gen_color(&ray, &self.objects.borrow(), 0);
        col = col + c;
      }
      col = col / self.super_samples as f64;
      *pixel = Color::from_vec3_gamma(col, 255, 2.2).to_u32();
    });
    BufferWrapper(buf)
  }

  fn gen_color(&self, ray: &Ray, shapes: &Vec<Box<dyn Shape>>, depth: u32) -> Vec3 {
    for shape in shapes.iter() {
      if let Some(hit_info) = shape.hit(ray, 0.001, std::f64::MAX) {
        if let Some((scattered, attenuation)) = shape.scatter(&ray, &hit_info) {
          if depth < self.max_scatter_depth {
            let c = self.gen_color(&scattered, shapes, depth + 1);
            return c * attenuation;
          }
        }
        //return (hit_info.get_normal().clone() + Vec3::new(1.0, 1.0, 1.0)).dir(0.5);
        return Vec3::zero_vector();
      }
    }
    let mut t: f64 = 0.5f64 * (ray.direction().normalize().get_z() + 1.0_f64);
    t = t.clamp(0.0, 1.0);
    Vec3::lerp(t, &Vec3::new(1.0, 1.0, 1.0), &Vec3::new(0.5, 0.7, 1.0))
  }
}
