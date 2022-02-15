use crate::base::color::{Color, WHITE};
use crate::base::vec::Vec3;
use crate::object::camera::Camera;
use crate::object::ray::Ray;
use crate::object::shape::Shape;

use std::borrow::Borrow;
use std::cell::RefCell;

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
  objects: RefCell<Vec<Box<dyn Shape>>>,
}

impl Scene {
  pub fn new(camera: Camera, width: u32, height: u32, super_samples: u32) -> Self {
    Scene {
      camera: camera,
      background_color: WHITE,
      width: width,
      height: height,
      super_samples: super_samples,
      objects: RefCell::new(vec![]),
    }
  }

  pub fn set_camera(&mut self, camera: Camera) {
    self.camera = camera;
  }

  pub fn set_background_color(&mut self, color: Color) {
    self.background_color = color;
  }

  pub fn add_object(&mut self, object: Box<dyn Shape>) {
    self.objects.borrow_mut().push(object);
  }

  pub fn render(&self) -> BufferWrapper {
    let mut buf = vec![0u32; (self.width * self.height) as usize];

    for i in 0..self.width {
      for j in 0..self.height {
        let mut col = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..self.super_samples {
          let u = (i as f64) / (self.width as f64);
          let v = (j as f64) / (self.height as f64);
          let ray = self.camera.get_ray(u, v);
          let c = Self::gen_color(&ray, &self.objects.borrow());
          col = col + c;
        }
        col = col / (self.super_samples as f64);
        buf[(i + j * self.width) as usize] = Color::from_vec3(col, 255).to_u32();
      }
    }
    BufferWrapper(buf)
  }

  fn gen_color(ray: &Ray, shapes: &Vec<Box<dyn Shape>>) -> Vec3 {
    for shape in shapes.iter() {
      if let Some(hit_info) = shape.hit(ray, 0.0, std::f64::MAX) {
        return (hit_info.get_normal().clone() + Vec3::new(1.0, 1.0, 1.0)).dir(0.5);
      }
    }
    let mut t: f64 = 0.5f64 * (ray.direction().get_y() + 1.0_f64);
    t = t.clamp(0.0, 1.0);
    return Vec3::lerp(t, &Vec3::new(0.5, 0.7, 1.0), &Vec3::new(1.0, 1.0, 1.0));
  }
}
