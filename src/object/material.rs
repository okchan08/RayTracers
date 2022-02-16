use crate::base::vec::Vec3;

#[derive(Debug, Copy, Clone)]
pub enum Material {
  Lambertian { albedo: Vec3 },
  Metal { albedo: Vec3 },
  Black,
}
