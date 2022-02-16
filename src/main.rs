use std::borrow::Borrow;
use std::path::Path;

use ray_tracers::base::vec::Vec3;
use ray_tracers::object::camera::Camera;
use ray_tracers::object::material::Material;
use ray_tracers::object::sphere::Sphere;
use ray_tracers::scene::Scene;

fn main() {
    const WIDTH: u32 = 1080;
    const HEIGHT: u32 = 720;
    const SAMPLING: u32 = 100;
    const MAX_SCATTER_DEPTH: u32 = 50;

    let lookfrom = Vec3::new(0.0, -20.0, 3.0);
    let lookat = Vec3::new(0.0, -1.0, 0.0);
    let dist = (lookfrom - lookat).norm();
    let camera = Camera::new(
        lookfrom,                         // lookfrom
        lookat,                           // lookat
        Vec3::new(0.0, 1.0, -0.5),        // vup
        20.0,                             // vfov
        (WIDTH as f64) / (HEIGHT as f64), // aspect
        0.1,                              // aperture
        dist,                             // dist_to_focus
    );

    let mut scene = Scene::new(camera, WIDTH, HEIGHT, SAMPLING, MAX_SCATTER_DEPTH);
    scene.add_object(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.75),
        1.5,
        "sphere 1".to_string(),
        Material::Lambertian {
            albedo: Vec3::new(0.9, 0.7, 0.4),
        },
    )));
    scene.add_object(Box::new(Sphere::new(
        Vec3::new(2.0, 0.0, 0.4),
        0.8,
        "sphere 2".to_string(),
        Material::Metal {
            albedo: Vec3::new(0.7, 0.3, 0.3),
            fuzzy: 0.0,
        },
    )));
    scene.add_object(Box::new(Sphere::new(
        Vec3::new(-3.5, 2.0, 0.15),
        0.3,
        "sphere 3".to_string(),
        Material::Metal {
            albedo: Vec3::new(0.0, 0.3, 0.8),
            fuzzy: 0.1,
        },
    )));
    scene.add_object(Box::new(Sphere::new(
        Vec3::new(-3.0, 1.0, 0.75),
        1.5,
        "sphere 4".to_string(),
        Material::Dielectric {
            refraction_index: 1.5,
        },
    )));
    scene.add_object(Box::new(Sphere::new(
        Vec3::new(-2.5, 4.0, 0.15),
        0.3,
        "sphere 5".to_string(),
        Material::Metal {
            albedo: Vec3::new(0.8, 0.1, 0.1),
            fuzzy: 0.0,
        },
    )));
    scene.add_object(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -100.0),
        100.5,
        "floor".to_string(),
        Material::Lambertian {
            albedo: Vec3::new(0.8, 0.8, 0.0),
        },
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
