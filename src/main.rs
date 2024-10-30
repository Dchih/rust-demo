#![allow(dead_code)]
mod closure;
mod helloworld;
mod generic;
mod guess_number;
mod process_control;
mod life_circle;

fn test_mod() {
    helloworld::sayhi();
    helloworld::nested::nested_fn();
}


fn test_generic_in_method() {
    let p1 = generic::method::Point {x: 5, y: 10.4};
    let p2 = generic::method::Point {x: "hello", y: "world"};
    let p3 = p1.mixup(p2);
    let p4 = generic::method::Point { x: 1, y: 2};
    let p5 = generic::method::Point { x: 3, y: 4};
    let p6 = generic::method::Point { x: "5", y: "6"};
    // p4.mixup(p3);
    p4.print_ur_point_x(p5);
    p4.print_ur_point_x(p6);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn main() {
    life_circle::exec_all_fn();
}
