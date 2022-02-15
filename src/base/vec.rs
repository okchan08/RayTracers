use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
  x: f64,
  y: f64,
  z: f64,
}

impl Vec3 {
  pub fn new(x: f64, y: f64, z: f64) -> Self {
    Vec3 { x: x, y: y, z: z }
  }

  pub fn from_one(v: f64) -> Self {
    Vec3 { x: v, y: v, z: v }
  }

  pub fn unit_vector() -> Self {
    Self::from_one(1.0)
  }

  pub fn zero_vector() -> Self {
    Self::from_one(0.0)
  }

  pub fn gen_random_vector() -> Self {
    let unix_time = std::time::SystemTime::now()
      .duration_since(std::time::SystemTime::UNIX_EPOCH)
      .expect("failed to get UNIX time")
      .as_nanos();

    let mut rng = StdRng::seed_from_u64((unix_time & (std::u64::MAX - 1) as u128) as u64);

    Vec3 {
      x: rng.gen(),
      y: rng.gen(),
      z: rng.gen(),
    }
  }

  pub fn gen_random_vector_in_unit_shpere() -> Self {
    let mut p: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    // 単位円の内部にある(=長さが1^2以下)のベクトルが生成されるまでサンプリング
    while p.norm() >= 1.0 {
      p = Vec3::gen_random_vector() * 2.0 - Vec3::from_one(1.0);
    }
    p
  }

  pub fn norm(&self) -> f64 {
    (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
  }

  pub fn dir(&self, t: f64) -> Self {
    Vec3 {
      x: self.x * t,
      y: self.y * t,
      z: self.z * t,
    }
  }

  pub fn normalize(&self) -> Self {
    let n = self.norm();
    Vec3 {
      x: self.x / n,
      y: self.y / n,
      z: self.z / n,
    }
  }

  pub fn to_tuple(&self) -> (f64, f64, f64) {
    (self.x, self.y, self.z)
  }

  pub fn dot(&self, other: &Self) -> f64 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }

  pub fn cross(&self, other: &Self) -> Self {
    Vec3 {
      x: self.y * other.z - self.z * other.y,
      y: self.z * other.x - self.x * other.z,
      z: self.x * other.y - self.y * other.x,
    }
  }

  pub fn get_x(&self) -> f64 {
    self.x
  }

  pub fn get_y(&self) -> f64 {
    self.y
  }

  pub fn get_z(&self) -> f64 {
    self.z
  }

  pub fn lerp(t: f64, a: &Self, b: &Self) -> Self {
    assert!(0.0 <= t && t <= 1.0);
    a.dir(1.0 - t) + b.dir(t)
  }
}

impl PartialEq for Vec3 {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y && self.z == other.z
  }
}

impl Add for Vec3 {
  type Output = Vec3;
  fn add(self, other: Self) -> Vec3 {
    Vec3 {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl Sub for Vec3 {
  type Output = Vec3;
  fn sub(self, other: Self) -> Vec3 {
    Vec3 {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

impl Sub for &Vec3 {
  type Output = Vec3;
  fn sub(self, other: Self) -> Vec3 {
    Vec3 {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

impl Div<f64> for Vec3 {
  type Output = Vec3;
  fn div(self, other: f64) -> Vec3 {
    Vec3 {
      x: self.x / other,
      y: self.y / other,
      z: self.z / other,
    }
  }
}

impl Mul<f64> for Vec3 {
  type Output = Vec3;
  fn mul(self, other: f64) -> Self::Output {
    Vec3 {
      x: self.x * other,
      y: self.y * other,
      z: self.z * other,
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_norm() {
    assert_eq!(Vec3::new(1.0, 1.0, 1.0).norm(), 3.0_f64.sqrt());
    assert_eq!(Vec3::new(2.5, 1.0, 0.0).norm(), 7.25_f64.sqrt());
  }

  #[test]
  fn test_constructor() {
    assert_eq!(Vec3::unit_vector(), Vec3::new(1.0, 1.0, 1.0));
    assert_eq!(Vec3::zero_vector(), Vec3::new(0.0, 0.0, 0.0));
  }

  #[test]
  fn test_random_vector() {
    let random = Vec3::gen_random_vector_in_unit_shpere();
    assert_eq!(random.norm() <= 1.0, true);
  }

  #[test]
  fn test_dir() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v1.dir(0.5), Vec3::new(0.5, 1.0, 1.5));
  }

  #[test]
  fn test_normalize() {
    let v1 = Vec3::new(1.0, 1.0, 1.0);
    let s = 3.0_f64.sqrt();
    assert_eq!(v1.normalize(), Vec3::new(1.0 / s, 1.0 / s, 1.0 / s));
  }

  #[test]
  fn test_cross() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(v1.cross(&v2), Vec3::new(-3.0, 6.0, -3.0));
  }

  #[test]
  fn test_add() {
    let v1 = Vec3::new(1.0, 1.0, 1.0);
    let v2 = Vec3::new(2.0, 3.0, 4.0);
    assert_eq!(v1 + v2, Vec3::new(3.0, 4.0, 5.0));
  }

  #[test]
  fn test_sub() {
    let v1 = Vec3::new(1.0, 1.0, 1.0);
    let v2 = Vec3::new(2.0, 3.0, 4.0);
    assert_eq!(v1 - v2, Vec3::new(-1.0, -2.0, -3.0));
  }

  #[test]
  fn test_div() {
    assert_eq!(Vec3::new(1.0, 2.0, 3.0) / 2.0, Vec3::new(0.5, 1.0, 1.5));
  }

  #[test]
  fn test_mul() {
    assert_eq!(Vec3::new(1.0, 2.0, 3.0) * 1.5, Vec3::new(1.5, 3.0, 4.5));
  }
}
