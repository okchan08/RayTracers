use crate::object::camera::Camera;
use crate::base::color::Color;

pub struct Scene {
  camera: Camera
  background_color: Color;
  width: u32,
  heigh: u32,
}

impl Scene {
  pub fn new(width: u32, height: u32) -> Self {
    Scene {
      camera: Camera::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
      background_color: WHITE,
    }
  }

  pub fn set_camera(&mut self, camera: Camera) {
    self.camera = camera;
  }

  pub fn set_background_color(&mut self, color: Color) {
    self.background_color = color;
  }

  pub fn render(&self) -> [u8] {
    let mut buf = vec![0u32; (WIDTH * HEIGHT) as usize];

    for i in 0..self.width {
      for j in 0..self.height {
        let u = (i as f64) / (self.width as f64);
        let v = (j as f64) / (self.height as f64);
        let ray = camera.get_ray(u, v);
        let col = gen_color(&ray, &sphere);
        buf[(i + j * WIDTH) as usize] = rgba_to_u32(
          (col.get_x() * 255.0) as u8,
          (col.get_y() * 255.0) as u8,
          (col.get_z() * 255.0) as u8,
          0xFF,
        );
      }
    }
  }

  fn gen_color(ray: &Ray, sphere: &Sphere) -> Color {
    let t = sphere.hit_sphere(ray);
    if t > 0.0 {
        let n = (ray.at(t) - sphere.center()).normalize();
        return Color::from_vec3(n + Vec3::new(1.0, 1.0, 1.0), 255);
    }
    let t: f64 = 0.5f64 * (ray.direction().get_y() + 1.0_f64);
    Color::from_vec3(
        Vec3::lerp(t, &Vec3::new(0.5, 0.7, 1.0), &Vec3::new(1.0, 1.0, 1.0)),
        255,
    )
  }
}
