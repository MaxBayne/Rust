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

    println!("-----------------------------------------");

    //use some extensions

    let direction = Direction::Right;

    direction.print();
    direction.print_arabic();

    println!("-----------------------------------------");

    let tickets = vec![
        Ticket::Standard(200.0),
        Ticket::Backstage(350.0, "Ahmed".to_owned()),
        Ticket::Vip(500.0, "Dr.Khalid".to_owned()),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Standard(price) => println!("Standard Ticket with price = {:?}", price),
            Ticket::Backstage(price, holder) => println!(
                "Backstage Ticket with price = {:?},For Holder {:?}",
                price, holder
            ),
            Ticket::Vip(price, holder) => println!(
                "Vip Ticket with price = {:?},For Holder {:?}",
                price, holder
            ),
        }
    }
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

enum Discount {
    Percent(i32),
    Flat(i32),
}

enum Ticket {
    Standard(f64),
    Backstage(f64, String),
    Vip(f64, String),
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
