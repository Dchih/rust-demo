struct Empty;
struct Null;

pub trait DoubleDrop<T> {
  fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
  fn double_drop(self, _: T) {}
}

pub fn exec_all() {
  let empty = Empty;
  let null = Null;
  empty.double_drop(null);
}