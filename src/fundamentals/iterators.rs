#![allow(dead_code)]
#![allow(unused_assignments)]

pub fn run() {
    println!("================= Iterators =================");

    let numbers = vec![10, 20, 30, 40, 50];

    println!("--- loop throw collections ---");

    print_collection(&numbers);

    println!("--- using map with iterators ---");
    //loop throw current collection and change its items to add one and then return new collection
    let plus: Vec<i32> = numbers.iter().map(|number| number + 1).collect();

    print_collection(&plus);

    println!("--- using filter with iterators ---");

    let filter: Vec<_> = numbers.iter().filter(|&&num| num >= 30).collect();

    print_collection(&filter);

    println!("--- using take with iterators ---");

    let takes: Vec<_> = numbers.iter().take(3).collect();

    print_collection(&takes);

    println!("--- using find with iterators ---");

    let item_exist = numbers.iter().find(|&&item| item == 40);

    if item_exist.is_some() {
        println!("Number 40 exist inside list");
    } else {
        println!("Number 40 Not Exist");
    }

    println!("--- using count with iterators ---");

    let count_of_list = numbers.iter().count();

    println!("Count of Items inside List = {count_of_list}");

    println!("--- using last with iterators ---");

    let last_item = numbers.iter().last();

    println!("Last Item = {:?}", last_item);

    println!("--- using min/max with iterators ---");

    let min_value = numbers.iter().min();
    let max_value = numbers.iter().max();

    println!("Min Value = {:?} , Max Value = {:?}", min_value, max_value);
}

//print collection of Vectors
fn print_collection<T: std::fmt::Debug>(numbers: &Vec<T>) {
    for num in numbers.iter() {
        println!("Number = {:#?}", num);
    }
}
