#![allow(dead_code)]
#![allow(unused_assignments)]

pub fn run() {
    println!("================= Maps =================");

    //maps used to format data from shape to another using closures (anonumouse functons)

    //Convert Data From Option<String> to Option<User> using Map
    let user = get_user_name_by_id(300).map(|(userid, username)| User {
        id: userid,
        name: username,
    });

    match user {
        Some(user) => println!("Id={:?},Name={:?}", user.id, user.name),
        None => println!("User Not Exist"),
    }
}

struct User {
    id: i32,
    name: String,
}

fn get_user_name_by_id(id: i32) -> Option<(i32, String)> {
    match id {
        100 => return Some((100, "Ahmed".to_owned())),
        200 => return Some((200, "Khalid".to_owned())),
        300 => return Some((300, "Mostafa".to_owned())),
        400 => return Some((400, "Mona".to_owned())),
        500 => return Some((500, "Hoda".to_owned())),
        _ => return None,
    }
}
