pub fn if_let() {
  let _s = Some(2);
  let b: Option<i32> = None;

  if let Some(i) = b {
    println!("匹配到了i: {i}");
  } else if false {
    println!("解构失败")
  } else  {
    println!("默认分支")
  }
}