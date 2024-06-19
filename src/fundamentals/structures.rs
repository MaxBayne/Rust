#![allow(dead_code)]
#![allow(unused_assignments)]

pub fn run() {
    println!("================= Structure =================");

    let emp = EmployeeRecord {
        name: "Mohammed Salah".to_string(),
        age: 38,
        salary: 15000.0,
    };

    emp.print();

    //using some extenstions
    let emp2 = EmployeeRecord::new("khalid salah".to_string(), 2, 0.0);

    emp2.print();
}

//Declare Structure ============================================
struct EmployeeRecord {
    name: String,
    age: i32,
    salary: f32,
}

//Implement Some Extenstions inside Structure ==================
//Set some functions inside Structure like extensions
impl EmployeeRecord {
    fn new(name: String, age: i32, salary: f32) -> Self {
        return Self { name, age, salary };
    }

    fn print(&self) {
        println!("----------------------------------");
        println!("Employee Name is {:?}", self.name);
        println!("Employee Age is {:?}", self.age);
        println!("Employee Salary is {:?}", self.salary);
        println!("----------------------------------");
    }
}
