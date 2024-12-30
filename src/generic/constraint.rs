use std::fmt::Display;
use std::fmt::Debug;

fn printer<T: Display>(t: T) {
  println!("{}", t);
}

// concrete: to implement some thing; make something real
fn concrete() {
  struct S<T: Display>(T);
  // let s = S(vec![1]);
}

fn use_fn_from_trait() {
  
}