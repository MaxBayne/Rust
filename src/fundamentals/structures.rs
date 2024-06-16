#![allow(dead_code)]
#![allow(unused_assignments)]

pub fn run() {
    println!("================= Structure =================");

    let emp = EmployeeRecord {
        name: "Mohammed Salah".to_string(),
        age: 38,
        salary: 15000.0,
    };

    print_employee(emp);
}

struct EmployeeRecord {
    name: String,
    age: i32,
    salary: f32,
}

fn print_employee(emp: EmployeeRecord) {
    println!("----------------------------------");
    println!("Employee Name is {:?}", emp.name);
    println!("Employee Age is {:?}", emp.age);
    println!("Employee Salary is {:?}", emp.salary);
    println!("----------------------------------");
}
