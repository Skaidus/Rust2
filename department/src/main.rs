use std::collections::HashMap;
use std::io;
fn main() {
    let mut my_enterprise = Enterprise::new();
    loop {

        
        println!("Please choose your command:\n1: Add EMPLOYEE to DEPARTMENT\n2: Get all info");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the command");
        let cmd: u32 = guess.trim().parse().expect("Please type a number!");
        match cmd {
            1 => {
                println!("Please add the employee:");
                let mut employee = String::new();
                io::stdin()
                    .read_line(&mut employee)
                    .expect("Failed to read the employee");

                    println!("Please add the department:");
                let mut department = String::new();
                io::stdin()
                        .read_line(&mut department)
                        .expect("Failed to read the department");
                my_enterprise.add_employee(department, employee);
            },
            2 => {
                my_enterprise.print_all();
            },
            _=> println!("Unrecognized command!")
        }
    }
    
}

struct Enterprise {
    hierarchy : HashMap <String, Vec<String>> 
}

impl Enterprise {

    fn new()-> Enterprise{
        Enterprise { hierarchy : HashMap::new()}
    }

    fn add_employee(&mut self, department: String , employee : String){
        let current_department = self.hierarchy.entry(department).or_insert(Vec::new());
        current_department.push(employee);
        current_department.sort_unstable();
    }

    fn print_all(&self){
        if self.hierarchy.len() > 0 {
            for department in self.hierarchy.keys(){
                println!("Departamento {}=============", department);
                for employee in &self.hierarchy[department]{
                    print!("||Empleado {}", employee);
                }
                println!("=============");
            }
        } else {
            println!("El departamento está vacío...")
        }

    }
}

/* 


add {} to {} =>
get all => 

hash of vectors
vector sorted 

struct enterprise
    hash of vectors

    add_employee(employee, department)
        (if new department then insert vector)
            department : employee
        add to vector sorted  

    get_all
        print

*/