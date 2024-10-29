pub fn natural_add_2_100 (num: &mut i32) {
  'tag: loop {
    *num += 1;
    if *num >= 100 { 
      break 'tag
    }
  }
}

pub fn return_loop_result(n: &mut i32) -> i32 {
  loop {
    if *n >= 10 {
      break *n
    }
  }
}