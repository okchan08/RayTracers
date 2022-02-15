use crate::base::math::RAND_GEN;
use crate::base::vec::Vec3;
use crate::object::ray::Ray;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

// カメラ(=視界)を表す構造体
pub struct Camera {
  // カメラの位置
  origin: Vec3,
  // カメラの向き
  uvw: (Vec3, Vec3, Vec3),
  // カメラが持つ画角の左下位置
  lower_left_coner: Vec3,
  // 水平方向
  horizontal: Vec3,
  // 垂直方向
  vertical: Vec3,
  // レンズの半径
  lense_radius: f64,
}

impl Camera {
  // lookfrom: カメラ配置位置
  // lookat: 視線を向ける点
  // vfov: vertical_fov、つまり縦方向の視野角
  // vup: 上向き（ロール回転を固定する必要があるため）
  // aspect: アス比（横/縦）
  // aperture: 口径＝レンズの大きさ、大きいと取り込む光は増え、ボケる
  // focus_dist: 焦点距離 //
  pub fn new(
    lookfrom: Vec3,
    lookat: Vec3,
    vup: Vec3,
    vfov: f64,
    aspect: f64,
    aperture: f64,
    focal_dist: f64,
  ) -> Self {
    let lense_radius = aperture / 2.0;
    // 視野角をラジアンへ変換
    let theta = vfov * std::f64::consts::PI / 180.0;
    let half_height = (theta / 2.0).tan();
    let half_width = aspect * half_height;
    let origin = lookfrom;
    let w = (lookfrom - lookat).normalize();
    let u = vup.cross(&w).normalize();
    let v = w.cross(&u);
    let lower_left_coner =
      origin - u.dir(half_width * focal_dist) - v.dir(half_height * focal_dist) - w.dir(focal_dist);
    let horizontal = u.dir(2.0 * half_width * focal_dist);
    let vertical = v.dir(2.0 * half_height * focal_dist);
    Self {
      origin: origin,
      lower_left_coner: lower_left_coner,
      horizontal: horizontal,
      uvw: (u, v, w),
      vertical: vertical,
      lense_radius: lense_radius,
    }
  }

  pub fn get_ray(&self, s: f64, t: f64) -> Ray {
    //Ray::new(
    //  self.origin.clone(),
    //  self.uvw.2 + self.uvw.0.dir(u) + self.uvw.1.dir(v) - self.origin.clone(),
    //)
    let rd = Self::random_in_unit_disk().dir(self.lense_radius);
    let offset = self.uvw.0.dir(rd.get_x()) + self.uvw.1.dir(rd.get_y());
    Ray::new(
      self.origin.clone() + offset,
      self.lower_left_coner + self.horizontal.dir(s) + self.vertical.dir(t)
        - (self.origin + offset),
    )
  }

  // 単位円上の点でかつある程度内側に近い点を返す
  fn random_in_unit_disk() -> Vec3 {
    let mut p: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    // 単位円の内部にある(=長さが1^2以下)のベクトルが生成されるまでサンプリング
    while p.dot(&p) >= 1.0 {
      p = (Vec3::new(
        RAND_GEN.lock().unwrap().gen(),
        RAND_GEN.lock().unwrap().gen(),
        0.0,
      ) - Vec3::new(1.0, 1.0, 0.0))
      .dir(2.0);
    }
    return p;
  }
}
