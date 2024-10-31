// trait life time circle as similar as method
// attention: impl can own life time circle mark

#[derive(Debug)]
struct Borrowed<'a> {
  x: &'a i32,
}

// unsafe
impl<'a> Default for Borrowed<'a> {
  fn default() -> Self {
      Self {
        x: &10
      }
  }
}