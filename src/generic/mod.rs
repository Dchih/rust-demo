use std::ops::Add;
use std::cmp::PartialOrd;
use std::marker::Copy;

pub mod method;
mod implementation;

// 对泛型进行约束，使其能进行函数中的运算
fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// 对泛型进行约束，使其能进行函数中的运算
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// AI 版本 copy
// fn largest<T: PartialOrd + Copy>(list: &[T]) -> Option<T> {
//     list.iter().copied().max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
// }

// AI 版本 ref
// fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
//     list.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
// }

pub fn exec_all() {
    let x = implementation::Val { val: 32f64 };
    let y = implementation::GenVal { gen_val: 32i32 };

    println!("Val.val is: {}, Gen_Val.gen_val is: {}", x.value(), y.generate_value(23f64))
}