#![allow(dead_code)]
#![allow(unused_assignments)]

pub fn run() {
    println!("================= Tuples =================");

    //Using Tuple as Returned Data Type for Function
    let emp = create_employee_info();

    //Using Tuple as Input Parameters for Function
    print_employee_info(emp);

    //Using Tuple as Anonymouse Data Value
    let student_info = ("Ahmed Ali", 18, "Class A");

    println!("Student Name = {}", student_info.0);
    println!("Student Age = {}", student_info.1);
    println!("Student Class = {}", student_info.2);
}

//Using Tuple as Returned Data Type for Function
fn create_employee_info() -> (&'static str, i32, f64) {
    return ("Mohammed Salah", 38, 15000.0);
}

//Using Tuple as Input Parameters for Function
fn print_employee_info(info: (&str, i32, f64)) {
    let (name, age, salary) = info;

    println!("----------------------------------");
    println!("Employee Name is {}", name);
    println!("Employee Age is {}", age);
    println!("Employee Salary is {}", salary);
    println!("----------------------------------");
    println!("Employee Name is {}", info.0);
    println!("Employee Age is {}", info.1);
    println!("Employee Salary is {}", info.2);
    println!("----------------------------------");
}
