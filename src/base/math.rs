use once_cell::sync::Lazy;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::sync::Mutex;
pub fn clamp<T>(v: T, min: T, max: T) -> T
where
  T: std::cmp::Ord,
{
  v.max(min).min(max)
}

static RAND_GEN: Lazy<Mutex<StdRng>> = Lazy::new(|| Mutex::new(StdRng::seed_from_u64(125130568)));

pub fn get_random() -> f64 {
  let x: f64 = RAND_GEN.lock().unwrap().gen();
  x
}
