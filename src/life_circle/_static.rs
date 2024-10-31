// 2 ways to product static life time circle
static NUM: i32 = 1;

pub fn stat_str() {
  {
    let static_string = "this is a static string.";

    // 当 `static_string` 离开作用域时，该引用不能再使用，不过
    // 数据仍然存在于二进制文件里面。
  }
  // println!("static string: {static_string}");
}
