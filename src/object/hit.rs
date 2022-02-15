use crate::base::vec::Vec3;
use crate::object::material::Material;

#[derive(Debug)]
pub struct HitInfo {
  // 光線のパラメタ
  t: f64,
  // 光線が物体と衝突した位置
  position: Vec3,
  // 衝突した位置の法線方向
  normal: Vec3,
  // 衝突した物体の材質情報
  hit_material: Material,
}

impl HitInfo {
  pub fn new(t: f64, position: Vec3, normal: Vec3, hit_material: Material) -> Self {
    HitInfo {
      t: t,
      position: position,
      normal: normal,
      hit_material: hit_material,
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

  pub fn get_hit_material(&self) -> &Material {
    &self.hit_material
  }
}
