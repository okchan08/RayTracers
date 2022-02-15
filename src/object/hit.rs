use crate::base::vec::Vec3;

#[derive(Debug)]
pub struct HitInfo {
  // 光線のパラメタ
  t: f64,
  // 光線が物体と衝突したい位置
  position: Vec3,
  // 衝突した位置の法線方向
  normal: Vec3,
}

impl HitInfo {
  pub fn new(t: f64, position: Vec3, normal: Vec3) -> Self {
    HitInfo {
      t: t,
      position: position,
      normal: normal,
    }
  }

  pub fn get_t(&self) -> f64 {
    self.t
  }

  pub fn get_poisition(&self) -> &Vec3 {
    &self.position
  }

  pub fn get_normal(&self) -> &Vec3 {
    &self.normal
  }
}
