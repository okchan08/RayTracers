use crate::base::vec::Vec3;

pub struct Ray {
  origin: Vec3,
  direction: Vec3,
}

impl Ray {
  pub fn new(origin: Vec3, direction: Vec3) -> Self {
    Ray {
      origin: origin,
      direction: direction,
    }
  }

  pub fn at(&self, t: f64) -> Vec3 {
    self.origin + self.direction.dir(t)
  }

  pub fn origin(&self) -> &Vec3 {
    &self.origin
  }

  pub fn direction(&self) -> &Vec3 {
    &self.direction
  }
}

#[cfg(test)]
mod ray_test {
  use super::*;
  #[test]
  fn test_at() {
    let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(ray.at(2.0), Vec3::new(2.0, 4.0, 6.0));
  }
}
