#![allow(dead_code)]
#![allow(unused_assignments)]

pub fn run() {
    println!("================= Results =================");

    let check_result = check_website_available("hello");

    match check_result {
        Ok(result) => println!("success => {:?}", result),
        Err(e) => println!("An Error Occured => {:?}", e),
    }

    let gender = set_gender("male");

    match gender {
        Ok(Gender::Male) => println!("your gender is male"),
        Ok(Gender::Female) => println!("your gender is female"),
        Ok(Gender::Other) => println!("your gender is other"),
        Err(e) => println!("error Occured {:?}", e),
    }
}

fn check_website_available(url: &str) -> Result<bool, String> {
    if url.is_empty() {
        return Err("URL is Empty".to_owned());
    }

    return Ok(true);
}

fn set_gender(gender: &str) -> Result<Gender, String> {
    //let selected_gender = gender.to_lowercase();

    let selected_gender = gender;

    match selected_gender {
        "male" => Ok(Gender::Male),
        "female" => Ok(Gender::Female),
        "other" => Ok(Gender::Other),
        _ => Err("you must select gender !".to_owned()),
    }
}

enum Gender {
    Male,
    Female,
    Other,
}
