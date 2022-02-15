use std::borrow::Borrow;
use std::path::Path;

use ray_tracers::base::vec::Vec3;
use ray_tracers::object::camera::Camera;
use ray_tracers::object::sphere::Sphere;
use ray_tracers::scene::Scene;

fn main() {
    const WIDTH: u32 = 680;
    const HEIGHT: u32 = 460;
    const SAMPLING: u32 = 100;

    let lookfrom = Vec3::new(12.0, 2.08, 2.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus: f64 = 10.0;
    let aperture: f64 = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (WIDTH as f64) / (HEIGHT as f64),
        aperture,
        dist_to_focus,
    );

    let mut scene = Scene::new(camera, WIDTH, HEIGHT, SAMPLING);
    scene.add_object(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        "sphere 1".to_string(),
    )));
    scene.add_object(Box::new(Sphere::new(
        Vec3::new(-20.0, 0.0, -4.0),
        0.2,
        "sphere 2".to_string(),
    )));
    scene.add_object(Box::new(Sphere::new(
        Vec3::new(-70.0, -10.0, 0.0),
        1.5,
        "sphere 3".to_string(),
    )));
    let buf = scene.render();

    image::save_buffer(
        &Path::new("test.png"),
        buf.borrow(),
        WIDTH,
        HEIGHT,
        image::ColorType::Rgba8,
    )
    .expect("here");
}
