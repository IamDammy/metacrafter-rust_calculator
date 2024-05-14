//Write a simple program in Rust to create a struct of an “Employee” containing the following variables:
    //Employee Name
    //Employee Salary
    //Employee ID
    //Employee Type (JUNIOR ENGINEER, SENIOR ENGINEER)
//Also create a function to add salaries to the employee type.
//If it’s a JUNIOR ENGINEER, give them 50000 and if it’s SENIOR ENGINEER, assign their salary to 60000.

#[derive(Debug)]
enum EmployeeType {
    JuniorEngineer,
    SeniorEngineer,
}
#[derive(Debug)]
struct Employee {
    Name: String,
    Salary: f64, // salary should be of floating data type bit 64
    ID: u32,
    employee_type: EmployeeType,
}

impl Employee {
    fn add_salary(&mut self) {
        match self.employee_type {
            EmployeeType::JuniorEngineer => self.Salary = 50000.0,
            EmployeeType::SeniorEngineer => self.Salary = 60000.0,
        }
    }
}

fn main(){
    let mut employee_1 = Employee {
        Name: String::from("Ajayi Damola"),
        Salary: 0.0,
        ID: 1001,
        employee_type: EmployeeType::JuniorEngineer,
    };

    let mut employee_2 = Employee {
        Name: String::from("Ajayi Kingsley"),
        Salary: 0.0,
        ID: 1002,
        employee_type: EmployeeType::SeniorEngineer,
    };

    employee_1.add_salary();
    employee_2.add_salary();

    println!("Employee 1: {:#?}", employee_1);
    println!("Employee 2: {:#?}", employee_2);
}

