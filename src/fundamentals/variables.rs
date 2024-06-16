#![allow(dead_code)]
#![allow(unused_assignments)]



pub fn run()
{
    // Variables ----------------------------------->

    println!("================= Variables =================");

    //constant string (Imutable by default mean cant change it later)
    let first_name = "Mohaammed"; 
    let last_name  = "Salah";    

    //variable string (mutable mean can change it later)
    let mut welcome_message="hello world"; 
    welcome_message="Welcome Hello World";

    //constant character
    let prefex:char='d'; 

    //constant integer
    let age:i32=38; 

    //constant boolean
    let is_male:bool=true; 

    //constant decimal
    let salary:f32=25.5; 

    //constant float
    let long:f64=545454.45;

    println!("{}",first_name);
    println!("{}",last_name);
    println!("{}",prefex);
    println!("{}",age);
    println!("{}",is_male);
    println!("{}",salary);
    println!("{}",long);
    println!("{}",welcome_message);

    //Variables -----------------------------------<

    

}