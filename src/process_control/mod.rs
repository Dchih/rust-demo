pub mod if_else;
mod loop_1;


pub fn exec_all_fn() {
  let mut n = 1i32;
  n = if_else::if_else(&n);
  loop_1::natural_add_2_100(&mut n);
  println!("n: {n}")
}