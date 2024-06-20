#![allow(dead_code)]
#![allow(unused_assignments)]

pub fn run() {
    println!("================= Options =================");

    let student = Student {
        id: 100,
        name: "Khalid Salah".to_owned(),
        phone: Some("01091281295".to_owned()),
        address: None,
        active: Some(true),
    };

    if student.address != None {
        println!("Student Has Address");
    } else {
        println!("Student Not Has Address");
    }

    match student.phone {
        Some(phone) => println!("Student has Phone ({:?})", phone),
        None => println!("Student not have Phone Number"),
    }
}

struct Student {
    id: i32,
    name: String,
    phone: Option<String>,
    address: Option<String>,
    active: Option<bool>,
}
