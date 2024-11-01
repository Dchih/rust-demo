use std::fmt::Display;
pub struct Val {
  pub val: f64
}

pub struct GenVal<T> {
  pub gen_val: T
}

impl Val {
  pub fn value(self) -> f64 { self.val }
}

// impl Val {
//     fn value(&self) -> &f64 { &self.val }
// }

impl <T> GenVal<T> {
  pub fn generate_value<B>(&self, other: B) -> &T where B: Display { 
    println!("{other}");
    &self.gen_val
   }
}