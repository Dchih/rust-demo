use std::fmt::Display;

// 在方法中使用泛型
pub struct Point<T, U> {
  pub x: T,
  pub y: U
}

impl<T: Display, U: Display> Point<T, U> {
  pub fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
    Point {
      x: self.x,
      y: other.y
    }
  }

  pub fn print_ur_point_x<R: Display, S: Display>(&self, _other: Point<R, S>) {
    println!("x: {}, y: {}", self.x, self.y);
  }
}

// concrete type
struct A;
// concrete type 
struct S(A);
// generic
struct SGen<T>(T);

fn reg_fn(_s: S) {}

fn gen_spec_fn(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}
