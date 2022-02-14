use crate::base::vec::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
  r: u8,
  g: u8,
  b: u8,
  a: u8,
}

#[allow(dead_code)]
pub const WHITE: Color = Color {
  r: 255,
  g: 255,
  b: 255,
  a: 255,
};

pub const RED: Color = Color {
  r: 255,
  g: 0,
  b: 0,
  a: 255,
};

pub const GREEN: Color = Color {
  r: 0,
  g: 255,
  b: 0,
  a: 255,
};

impl Color {
  pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
    Color {
      r: r,
      g: g,
      b: b,
      a: a,
    }
  }

  pub fn to_u32(&self) -> u32 {
    let r = self.r as u32;
    let g = (self.g as u32) << 8;
    let b = (self.b as u32) << 16;
    let a = (self.a as u32) << 24;
    a | r | g | b
  }

  pub fn from_vec3(from: Vec3, alpha: u8) -> Self {
    let n = from.normalize();
    Color {
      r: (n.get_x() * 255.0) as u8,
      g: (n.get_y() * 255.0) as u8,
      b: (n.get_z() * 255.0) as u8,
      a: alpha,
    }
  }
}
