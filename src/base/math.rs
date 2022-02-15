pub fn clamp<T>(v: T, min: T, max: T) -> T
where
  T: std::cmp::Ord,
{
  v.max(min).min(max)
}
