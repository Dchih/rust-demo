pub fn easy_match(number: &i32) {
  match number {
    1 => println!("这是1"),
    2 | 3 | 4 | 5 => println!("2 , 3, 4, 5"),
    6..10 => println!("6-10"),
    _ => println!("大于10")
  }
}

// destructor
pub fn tuple_destruction(t: &(i32, i32, i32)) {
  match t {
    (0, x, y) => println!("x: {x}, y: {y}"),
    (1, ..) => println!("don't care rest value but first"),
    _ => println!("the rest situations")
  }
}

pub enum Color {
  Red,
  Blue,
  Green,
  RGB(u32, u32, u32),
  HSV(u32, u32, u32),
  HSL(u32, u32, u32),
  CMY(u32, u32, u32),
  CMYK(u32, u32, u32, u32),
}

pub fn enum_destruction(e: &Color) {
  match e {
    Color::Red => println!("Red"),
    Color::Blue => println!("Blue"),
    _ => println!("Others")
  }
}

// pointer & reference
pub fn pointer_ref_destruction() {
  let reference = &4;
  match reference {
    & val => println!("val is {val}"),
  }
  // or 
  match *reference {
    val => println!("val is {val}"),
  }

  let _not_a_reference = 3;
  let _is_a_reference = 4;

  let value = 5;
  let mut mut_value = 6;

  match value {
    ref val => println!("Got a reference to a value: {:?}", val),
  }
  // or
  match value {
    ref val => println!("Got a reference to a value: {:?}", *val),
  }
  match mut_value {
    ref mut value => {
      *value += 10;
      println!("Got a reference to a value: {:?}", value);
    }
  }
}

// struct
pub fn struct_destruction() {
  struct Foo {x: (u32, u32), y: u32 }
  let foo = Foo {x: (1, 2), y: 3};
  let Foo {x: (a, b), y: s} = foo;
  println!("a: {a}, b: {b}, s: {s}");
}

// guard sentence
pub fn guard_sen() {
  let pair = (2, -2);
  match pair {
    (x, y) if x == y => println!("元组(x, y), x == y"),
    (x, y) if x + y == 0 => println!("x y 互为相反数"),
    (x, _) if x % 2 == 0 => println!("x 为 偶数"),
    _ => println!("others")
  }
}

// bind @
pub fn bind_in_match(num: &i32) {
  match num {
    0 => println!("this is 0"),
    n @ 1..=10 => println!("1 - 10: {n}"),
    _ => println!("其余")
  }
}

// use bind to destruct enum
fn some_num() -> Option<i32> {
  Some(42)
}
pub fn bind_de_enum (num: &Option<i32>) {
  match num {
      Some(n @ 42) => println!("匹配到：{n}"),
      Some(n) => println!("匹配到：{n}"),
      None => println!("others"),
  }
}