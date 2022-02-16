use crate::base::vec::Vec3;
use crate::object::material::Material;
use crate::object::ray::Ray;

#[derive(Debug)]
pub struct HitInfo {
  // 光線のパラメタ
  t: f64,
  // 光線が物体と衝突した位置
  position: Vec3,
  // 衝突した位置の法線方向
  normal: Vec3,
  // 衝突点から外側に光線が出ているか
  front_face: bool,
  // 衝突した物体の材質情報
  hit_material: Material,
}

impl HitInfo {
  pub fn new(t: f64, position: Vec3, normal: Vec3, hit_material: Material, hit_ray: &Ray) -> Self {
    let front_face = hit_ray.direction().dot(&normal) < 0.0;
    HitInfo {
      t: t,
      position: position,
      normal: normal,
      hit_material: hit_material,
      front_face: front_face,
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

  pub fn set_front_face(&mut self, ray: &Ray, outward_normal: &Vec3) {
    let front_face = ray.direction().dot(outward_normal) < 0.0;
    self.front_face = front_face;
    self.normal = if front_face {
      outward_normal.clone()
    } else {
      outward_normal.clone() * (-1.0)
    };
  }

  pub fn front_face(&self) -> bool {
    self.front_face
  }
}
