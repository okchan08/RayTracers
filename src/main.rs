use std::borrow::Borrow;
use std::path::Path;

use ray_tracers::config::config::Config;
use ray_tracers::scene::Scene;

fn main() {
    let config = Config::from_yaml(Path::new("./example_yaml/example001.yaml")).unwrap();
    let scene = Scene::build_from_config(&config);
    let buf = scene.render();

    image::save_buffer(
        &Path::new(config.output_name()),
        buf.borrow(),
        config.render_config().width,
        config.render_config().height,
        image::ColorType::Rgba8,
    )
    .expect("here");
}
