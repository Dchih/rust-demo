pub fn methods_ref<'a>(_x: &'a i32) {
  println!("a's life is longer than the method.")
}

pub fn method_mut_ref<'a>(x: &'a mut i32) {
  *x += 2
}

pub fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
  println!("`print_multi`: x is {}, y is {}", x, y);
}

pub fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
  x
}

// null pointer example
pub fn invalid_output<'a>(x: &'a i32) -> String {
  // cannot return reference to temporary value
  // returns a reference to data owned by the current function
  // return type: &'a String
  // return &String::from("x")
  let x_str = x.to_string();
  println!("x_str is: {x_str}");
  String::from("x")
}