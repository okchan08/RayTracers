use std::borrow::Borrow;
use std::path::Path;

use ray_tracers::base::color::{Color, GREEN, RED, WHITE};
use ray_tracers::base::vec::Vec3;
use ray_tracers::object::camera::Camera;
use ray_tracers::object::ray::Ray;
use ray_tracers::object::sphere::Sphere;

struct BufferWrapper(Vec<u32>);

impl Borrow<[u8]> for BufferWrapper {
    fn borrow(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.0.as_ptr() as *const u8, self.0.len() * 4) }
    }
}

fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 400;

    let vec_x = Vec3::new(4.0, 0.0, 0.0);
    let vec_y = Vec3::new(0.0, 2.0, 0.0);
    let vec_z = Vec3::new(-2.0, -1.0, -1.0);
    let camera = Camera::new(vec_x, vec_y, vec_z);

    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let mut buf = BufferWrapper(vec![0u32; (WIDTH * HEIGHT) as usize]);

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let u = (i as f64) / (WIDTH as f64);
            let v = (j as f64) / (HEIGHT as f64);
            let ray = camera.get_ray(u, v);
            let col = gen_color(&ray, &sphere);
            buf.0[(i + j * WIDTH) as usize] = col.to_u32();
        }
    }

    image::save_buffer(
        &Path::new("test.png"),
        buf.borrow(),
        WIDTH,
        HEIGHT,
        image::ColorType::Rgba8,
    )
    .expect("here");
}

fn rgba_to_u32(red: u8, green: u8, blue: u8, alpha: u8) -> u32 {
    let a = (alpha as u32) << 24;
    let r = (red as u32) << 16;
    let g = (green as u32) << 8;
    let b = blue as u32;
    a | r | g | b
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

#[test]
fn test_rgb_to_u32() {
    assert_eq!(
        rgba_to_u32(123, 45, 24, 255),
        0xFF << 24 | (0x7B2D18 as u32)
    );
    assert_eq!(rgba_to_u32(4, 234, 97, 0), 0x04EA61);
    assert_eq!(rgba_to_u32(255, 255, 255, 255), u32::MAX);
}
