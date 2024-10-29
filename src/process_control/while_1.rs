pub fn while_loop(n: &mut i32) -> i32 {
  while *n < 102 {
    *n += 1;
  }
  return *n
}