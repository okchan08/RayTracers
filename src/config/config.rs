use crate::config::render_config::RenderConfig;
use crate::config::scene_config::SceneConfig;
use serde::{Deserialize, Serialize};
use serde_yaml;
use serde_yaml::Error;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
  output_name: String,
  render: RenderConfig,
  scene: SceneConfig,
}

impl Config {
  pub fn from_yaml(path: &Path) -> Result<Config, Error> {
    let f = std::fs::File::open(path).expect("failed to open");
    let d: Config = serde_yaml::from_reader(f)?;
    Ok(d)
  }

  pub fn output_name(&self) -> &str {
    self.output_name.as_str()
  }

  pub fn render_config(&self) -> &RenderConfig {
    &self.render
  }

  pub fn scene_config(&self) -> &SceneConfig {
    &self.scene
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_read() {
    let d = Config::from_yaml(&Path::new("./example_yaml/example001.yaml"));
    assert_eq!(d.is_ok(), true);
    println!("{:?}", d.unwrap());
  }
}
