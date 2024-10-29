// for range
pub fn for_range() {
  for m in 1..=100 {
    if m % 15 == 0 {
      println!("15 的公倍数")
    }
  }
}

// for & iterator

// iter borrow
pub fn for_iter(names: &mut [String; 4]) {
  for name in names.iter() {
    println!("{}", name);
  }
}

// iter_mut
pub fn for_iter_mut(names: &mut [String; 4]) {
  for name in names.iter_mut() {
    name.insert_str(0, "hi, ");
  }
}