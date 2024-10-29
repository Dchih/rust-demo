pub mod if_else;
mod loop_1;
mod while_1;
mod for_loop;


pub fn exec_all_fn() {
  let mut n = 1i32;
  n = if_else::if_else(&mut n);
  loop_1::natural_add_2_100(&mut n);
  let s = loop_1::return_loop_result(&mut n);
  let ns = while_1::while_loop(&mut n);


  // iter
  let mut names = [
    String::from("Tom"),
    String::from("Mary"),
    String::from("Gazy"), 
    String::from("Bill")
  ];
  // 在 Rust 中,& 和 &mut 是两种不同的引用:
  // & - 创建不可变引用,只能读取数据
  // &mut - 创建可变引用,可以修改数据
  // 这里我们需要可变引用的数组,因为:
  // 1. 数组中的每个元素都需要是可变的字符串引用(&mut str)
  // 2. 整个数组本身也需要是可变的(mut names)
  // 3. 传递给函数时需要可变借用(&mut names)
  for_loop::for_iter(&mut names);
  for_loop::for_iter_mut(&mut names);
  for name in names.into_iter() {
    println!("name: {name}");
  }
  println!("n: {n}, {s}, {ns}");
}