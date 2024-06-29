#![allow(dead_code)]
#![allow(unused_assignments)]

pub fn run() {
    println!("================= Closures =================");

    //its like anonumouse function

    let add = |a: i32, b: i32| -> i32 {
        return a + b;
    };

    let sub = |a, b| a - b;

    let add_two_number = add(100, 50);

    println!("Add Two Number = {add_two_number}");

    let subtract_two_number = sub(50, 10);

    println!("Sub Two Number = {subtract_two_number}");
}
