#![allow(dead_code)]
#![allow(unused_assignments)]



pub fn run()
{
    println!("================= Functions =================");


    //Call Method
    print("Iam From Method");

    //Call Function
    let result_of_add:i32 = add(10,25);
    println!("{:?}",result_of_add);
}


//Example For Method
fn print(message: &str)
{
    println!("{}",message);
}

//Example For Function
fn add(number1:i32,number2:i32)->i32
{
    return number1+number2;
}
