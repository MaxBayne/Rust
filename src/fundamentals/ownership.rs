#![allow(dead_code)]
#![allow(unused_assignments)]

pub fn run() {
    println!("================= Ownership =================");

    //ownership is rust system to manage memory and avoid memory leak
    //its make every variable created to set its ownership to the function that create it
    //when function block finish then the function will clear its owned function from memory
    //to avoid changing the ownership of variable we pass variables by ref (borrow)

    //creating variable (user_info) will made its ownership to function run
    let user_info = UserInfo {
        id: 1,
        name: "Mohammed Salah".to_string(),
        role: UserType::Admin,
    };

    //pass user info by ref to avoid change its owner ship from run function to print_user_name function
    print_user_name(&user_info);
    print_user_role(&user_info);
}

//make function accept parameter by ref (borrow) not to owne it
fn print_user_name(userinfo: &UserInfo) {
    println!("({})=>{}", userinfo.id, userinfo.name);
}

//make function accept parameter by ref (borrow) not to owne it
fn print_user_role(userinfo: &UserInfo) {
    match userinfo.role {
        UserType::Admin => println!("({})=>its Admin Role", userinfo.id),
        UserType::Manager => println!("({})=>its Manager Role", userinfo.id),
        UserType::User => println!("({})=>its User Role", userinfo.id),
        UserType::Guest => println!("({})=>its Guest Role", userinfo.id),
    };
}

struct UserInfo {
    id: i32,
    name: String,
    role: UserType,
}

enum UserType {
    Admin,
    Manager,
    User,
    Guest,
}
