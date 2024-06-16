#![allow(dead_code)]
#![allow(unused_assignments)]

pub fn run() {
    println!("================= Structure =================");

    let emp = EmployeeRecord {
        name: "Mohammed Salah".to_string(),
        age: 38,
        salary: 15000.0,
    };

<<<<<<< HEAD
    print_employee(emp);
=======
    print(emp);
>>>>>>> 160ca0fb328588e525c390198ad13287f6a57a28
}

struct EmployeeRecord {
    name: String,
    age: i32,
    salary: f32,
}

<<<<<<< HEAD
fn print_employee(emp: EmployeeRecord) {
=======
fn print(emp: EmployeeRecord) {
>>>>>>> 160ca0fb328588e525c390198ad13287f6a57a28
    println!("----------------------------------");
    println!("Employee Name is {:?}", emp.name);
    println!("Employee Age is {:?}", emp.age);
    println!("Employee Salary is {:?}", emp.salary);
    println!("----------------------------------");
}
