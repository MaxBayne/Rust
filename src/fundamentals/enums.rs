#![allow(dead_code)]
#![allow(unused_assignments)]

pub fn run() {
    println!("================= Enum =================");

    let genre_type = Genre::Female;

    match genre_type {
        Genre::Male => println!("its male"),
        Genre::Female => println!("its female"),
        Genre::Other => println!("its other"),
    }

    //use some extensions

    let direction = Direction::Right;

    direction.print();
    direction.print_arabic();
}

//Declare Enums =====================================

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

//Set Some Functions inside Enums (Extensions)==========

impl Direction {
    fn print(&self) {
        match self {
            Direction::Up => println!("Up"),
            Direction::Down => println!("Down"),
            Direction::Right => println!("Right"),
            Direction::Left => println!("Left"),
        }
    }

    fn print_arabic(&self) {
        match self {
            Direction::Up => println!("اعلي"),
            Direction::Down => println!("اسفل"),
            Direction::Right => println!("يمين"),
            Direction::Left => println!("يسار"),
        }
    }
}
