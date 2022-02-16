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

static RAND_GEN: Lazy<Mutex<StdRng>> = Lazy::new(|| {
  let now = std::time::SystemTime::now();
  let unixtime = now
    .duration_since(std::time::SystemTime::UNIX_EPOCH)
    .expect("back to the future")
    .as_nanos();
  Mutex::new(StdRng::seed_from_u64(unixtime as u64))
});

pub fn get_uniform_random() -> f64 {
  let x: f64 = RAND_GEN.lock().unwrap().gen();
  x
}

pub fn get_random_in_range(lower: f64, upper: f64) -> f64 {
  let x: f64 = RAND_GEN.lock().unwrap().gen_range(lower..upper);
  x
}
