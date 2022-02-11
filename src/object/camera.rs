use crate::base::vec::Vec3;
use crate::object::ray::Ray;

pub struct Camera {
  origin: Vec3,
  uvw: (Vec3, Vec3, Vec3),
}

impl Camera {
  pub fn new(u: Vec3, v: Vec3, w: Vec3) -> Self {
    Camera {
      origin: Vec3::new(0.0, 0.0, 0.0),
      uvw: (u, v, w),
    }
  }

  pub fn from_aspect(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Self {
    //vec3 u, v, w;
    //halfH = tanf(radians(vfov)/2.0f);
    //float halfW = aspect * halfH;
    //m_origin = lookfrom;
    //w = normalize(lookfrom - lookat);
    //u = normalize(cross(vup, w));
    //v = cross(w, u);
    //m_uvw[2] = m_origin - halfW*u - halfH*v - w;
    //m_uvw[0] = 2.0f * halfW * u;
    //m_uvw[1] = 2.0f * halfH * v;
    let half_h = (vfov.to_radians() / 2.0).tan();
    let half_w = half_h * aspect;
    let w = (lookfrom - lookat).normalize();
    let u = vup.cross(&w).normalize();
    let v = w.cross(&u);
    Camera {
      origin: lookfrom,
      uvw: (
        u.dir(2.0f64 * half_w),
        v.dir(2.0f64 * half_h),
        lookfrom - w - u.dir(half_w) - v.dir(half_h),
      ),
    }
  }

  pub fn get_ray(&self, u: f64, v: f64) -> Ray {
    Ray::new(
      self.origin.clone(),
      self.uvw.2 + self.uvw.0.dir(u) + self.uvw.1.dir(v) - self.origin.clone(),
    )
  }
}
