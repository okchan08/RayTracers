use once_cell::sync::Lazy;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::sync::Mutex;
pub fn clamp<T>(v: T, min: T, max: T) -> T
where
  T: std::cmp::Ord,
{
  v.max(min).min(max)
}

pub static RAND_GEN: Lazy<Mutex<StdRng>> =
  Lazy::new(|| Mutex::new(StdRng::seed_from_u64(125130568)));
