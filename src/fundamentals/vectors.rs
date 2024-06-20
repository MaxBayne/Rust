#![allow(dead_code)]
#![allow(unused_assignments)]

pub fn run() {
    println!("================= Vector =================");

    let points = vec![10, 50, 150];

    for point in points {
        println!("{}", point);
    }

    println!("------------------------------------------");

    let mut numbers = Vec::new();

    numbers.push(100);
    numbers.push(200);
    numbers.push(300);
    numbers.push(400);
    numbers.push(500);

    numbers.pop();

    for number in &numbers {
        println!("Number = {:?}", number);
    }

    println!("Numbers Count = {:?}", numbers.len());
}
