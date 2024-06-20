#![allow(dead_code)]
#![allow(unused_assignments)]

pub fn run() {
    println!("================= Derive Functionalty =================");

    //you can add functionality to struct or enum by using derive keyward

    let user = User {
        email: "maxbayne@gmail.com".to_owned(),
        password: "123".to_owned(),
        role: Role::Admin,
        is_active: true,
    };

    println!("{:?}", Role::Admin);

    println!("{:?}", user);
}

#[derive(Debug)]
enum Role {
    Admin,
    Manager,
    User,
}

#[derive(Debug)]
struct User {
    email: String,
    password: String,
    role: Role,
    is_active: bool,
}
