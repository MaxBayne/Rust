#![allow(dead_code)]
#![allow(unused_assignments)]

pub fn run() {
    println!("================= IF Condition =================");

    let age = 30;

    if age > 20 {
        println!("Age > 20");
    } else if age < 50 {
        println!("Age < 50");
    } else {
        println!("Age > 50");
    }

    println!("================= Match Condition =================");

    let counter = 50;

    match counter {
        10 => println!("its 10"),
        20 => print!("its 20"),
        30 => print!("its 30"),
        40 => print!("its 40"),
        50 => print!("its 50"),
        _ => print!("its somthing else"),
    }
}
