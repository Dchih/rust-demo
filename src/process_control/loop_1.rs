pub fn natural_add_2_100 (num: &mut i32) {
  'tag: loop {
    *num += 1;
    if *num >= 100 { 
      break 'tag
    }
  }
}