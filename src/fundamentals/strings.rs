#![allow(dead_code)]
#![allow(unused_assignments)]

pub fn run() {
    println!("================= String =================");

    let name: &str = "Mohammed Salah";

    print_string(name);

    let profile = Profile {
        id: 100,
        name: String::from("mohammed salah"), //"mohammed salah".to_owned()
        phone: String::from("01091281295"),   //"01091281295".to_owned()
        age: 38,
    };

    print_profile(profile);
}

fn print_string(message: &str) {
    println!("Message = {}", message);
}

fn print_profile(profile: Profile) {
    println!("Id = {}", profile.id);
    println!("Name = {}", profile.name);
    println!("Phone = {}", profile.phone);
    println!("Age = {}", profile.age);
}

struct Profile {
    id: i32,
    name: String,
    phone: String,
    age: i32,
}
