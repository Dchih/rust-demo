#[derive(Debug)]
pub struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
pub struct NameBorowed<'a> {
  x: &'a i32,
  y: &'a i32, 
}

#[derive(Debug)]
enum Either<'a> {
  Num(i32),
  Ref(&'a i32),
}

pub fn exec_structs() {
  let x = 15;
  let _y = 18;

  let borrowed = Borrowed(&x);
  let named_borrowed = NameBorowed { x: &x, y: &_y};

  let number = Either::Num(x);
  let reference = Either::Ref(&x);

  println!("x is borrowed in {:?}", borrowed);
    println!("x and y are borrowed in {:?}", named_borrowed);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}