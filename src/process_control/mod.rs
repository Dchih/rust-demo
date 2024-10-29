pub mod if_else;
mod loop_1;
mod while_1;


pub fn exec_all_fn() {
  let mut n = 1i32;
  n = if_else::if_else(&mut n);
  loop_1::natural_add_2_100(&mut n);
  let s = loop_1::return_loop_result(&mut n);
  let ns = while_1::while_loop(&mut n);
  println!("n: {n}, {s}, {ns}")
}