#![allow(dead_code)]
#![allow(unused_assignments)]

pub fn run() {
    println!("================= Expressions =================");

    //Example 1 ----------------------------------

    let age = 50;

    let age_greater = age >= 50;

    println!("Age is Greater = {}", age_greater);

    //Example 2 ----------------------------------

    let genre_type = GenreType::Female;

    let is_male = match genre_type {
        GenreType::Male => true,
        GenreType::Female => false,
        _ => false,
    };

    println!("He is Male = {}", is_male);
}

enum GenreType {
    Male,
    Female,
    Other,
}
