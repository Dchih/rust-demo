// force transform
// long to short
// <'a: 'b, 'b> read as `'a 's life circle at least as long as 'b`

pub fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
  *first * *second
  // first * second
}

pub fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
  first
}

pub fn exec_transform_fn() {
  let first = &10;
  let second = &3;
  let result = multiply(first, second);
  println!("result: {result}");

  let r2 = choose_first(first, second);
  println!("first is : {r2}");
}