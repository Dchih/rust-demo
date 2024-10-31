use std::fmt::Debug;

/**
 * this shows how to define both life time circle & template
 */
#[derive(Debug)]
pub struct Ref<'a, T: 'a>(pub &'a T);

// impl<'a, T: 'a> Ref<'a, T> {
//   pub fn new(value: &'a T) -> Self {
//     Self(value)
//   }
// }

pub fn print_cons<T>(t: T) 
where T: Debug {
  println!("`print`: t is {:?}", t);
}


pub fn print_cons_life<'a, T>(t: &'a T) where 
T: Debug + 'a {
  println!("`print_ref`: t is {:?}", t);
}

