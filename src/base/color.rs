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
    //let n = from.normalize();
    Color {
      r: (from.get_x().clamp(0.0, 1.0) * 255.0) as u8,
      g: (from.get_y().clamp(0.0, 1.0) * 255.0) as u8,
      b: (from.get_z().clamp(0.0, 1.0) * 255.0) as u8,
      a: alpha,
    }
  }

  pub fn from_vec3_gamma(from: Vec3, alpha: u8, gamma_factor: f64) -> Self {
    let inv_gamma_factor = 1.0 / gamma_factor;
    let x = from.get_x().powf(inv_gamma_factor);
    let y = from.get_y().powf(inv_gamma_factor);
    let z = from.get_z().powf(inv_gamma_factor);
    Color {
      r: (x.clamp(0.0, 0.999) * 256.0) as u8,
      g: (y.clamp(0.0, 0.999) * 256.0) as u8,
      b: (z.clamp(0.0, 0.999) * 256.0) as u8,
      a: alpha,
    }
  }
}
