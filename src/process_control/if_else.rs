pub fn if_else(n:&mut i32) -> i32{
  // should return same type
  if *n > 10 {
    *n * 2
  } else if *n == 0 {
    1
  } else {
    *n * 4
  }
}