#![allow(dead_code)]
#![allow(unused_assignments)]

use std::collections::HashMap;

pub fn run() {
    println!("================= Hashmap =================");

    //its like dictionaly in another programming language like c#

    let mut items = HashMap::new();

    //Add Item inside Collection
    items.insert("chairs", 5);
    items.insert("beds", 3);
    items.insert("tables", 2);
    items.insert("couches", 0);
    items.insert("doors", 7);
    items.insert("windows", 4);
    items.insert("other", 2);

    //Remove item By Key
    items.remove("other");

    //Remove item By Key and return its value
    let value_of_windows = items.remove("windows");

    match value_of_windows {
        Some(value) => println!("you remove key (windows) with value = {value}"),
        None => println!("windows key Not Exist inside HashMap"),
    }

    //Loop throw Collection

    //Loop throw Keys and Values
    println!("");
    println!("Loop throw Keys and Values");
    println!("--------------------------");

    for (key, value) in items.iter() {
        println!("Key={key},Value={value}");
    }

    //Loop throw Keys Only
    println!("");
    println!("Loop throw Keys Only");
    println!("---------------------");
    for key in items.keys() {
        println!("Key={key}");
    }

    //Loop throw Values Only
    println!("");
    println!("Loop throw Values Only");
    println!("---------------------");
    for value in items.values() {
        println!("value={value}");
    }
}
