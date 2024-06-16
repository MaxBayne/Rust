#![allow(dead_code)]
#![allow(unused_assignments)]

pub fn run() {
    println!("================= Enum =================");

    let genre_type: Genre = Genre::Female;

    match genre_type {
        Genre::Male => println!("its male"),
        Genre::Female => println!("its female"),
        Genre::Other => println!("its other"),
    }
}

enum Direction {
    Up = 0,
    Down = 1,
    Right = 2,
    Left = 3,
}

enum Genre {
    Male,
    Female,
    Other,
}
