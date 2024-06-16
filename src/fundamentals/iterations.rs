#![allow(dead_code)]
#![allow(unused_assignments)]

pub fn run() {
    println!("================= Loop =================");

    let mut counter = 0;

    loop {
        //Condition For the Loop
        if counter == 10 {
            println!("{:?}", counter);
            break;
        }

        //Loop Code
        println!("{:?}", counter);

        //Decrement Loop Counter
        //counter = counter+1;
        counter += 1;
    }

    println!("================= While =================");

    while counter >= 0 {
        //Loop Code
        println!("{:?}", counter);

        counter -= 1;
    }
}
