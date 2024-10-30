pub fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

pub fn failed_borrow<'a>() {
  let _x = 42;
  // 显示标注会报错
  let _y = &_x;
}