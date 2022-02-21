use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct RenderConfig {
  pub width: u32,
  pub height: u32,
  pub sampling: u32,
  pub max_scatter_depth: u32,
}

impl RenderConfig {
  pub fn default() -> Self {
    Self {
      width: 680,
      height: 460,
      sampling: 1,
      max_scatter_depth: 50,
    }
  }
}
