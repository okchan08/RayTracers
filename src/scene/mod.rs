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
  objects: RefCell<Vec<Box<dyn Shape>>>,
}

impl Scene {
  pub fn new(camera: Camera, width: u32, height: u32) -> Self {
    Scene {
      camera: camera,
      background_color: WHITE,
      width: width,
      height: height,
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
    println!("objects: {}", self.objects.borrow().len());
    let mut buf = vec![0u32; (self.width * self.height) as usize];

    for i in 0..self.width {
      for j in 0..self.height {
        for obj in self.objects.borrow().iter() {
          let u = (i as f64) / (self.width as f64);
          let v = (j as f64) / (self.height as f64);
          let ray = self.camera.get_ray(u, v);
          let col = Self::gen_color(&ray, obj);
          buf[(i + j * self.width) as usize] = col.to_u32();
        }
      }
    }
    BufferWrapper(buf)
  }

  fn gen_color(ray: &Ray, shape: &Box<dyn Shape>) -> Color {
    if let Some(hit_info) = shape.hit(ray, 0.0, std::f64::MAX) {
      return Color::from_vec3(
        (hit_info.get_normal().clone() + Vec3::new(1.0, 1.0, 1.0)).dir(0.5),
        255,
      );
    }
    //let t = sphere.hit_sphere(ray);
    //if t > 0.0 {
    //  let n = (ray.at(t) - sphere.center()).normalize();
    //  return Color::from_vec3(n + Vec3::new(1.0, 1.0, 1.0), 255);
    //}
    let mut t: f64 = 0.5f64 * (ray.direction().get_y() + 1.0_f64);
    if t < 0.0 {
      t = 0.0;
    }
    if t > 1.0 {
      t = 1.0;
    }
    Color::from_vec3(
      Vec3::lerp(t, &Vec3::new(0.5, 0.7, 1.0), &Vec3::new(1.0, 1.0, 1.0)),
      255,
    )
  }
}
