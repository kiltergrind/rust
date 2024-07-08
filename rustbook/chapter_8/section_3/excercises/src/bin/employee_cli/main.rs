extern crate chapter8;

mod utils;

use std::io::{self, Write};
use chapter8::solutions::employee_management as employman;


fn main() {
    let mut company = employman::Company::new();
    let mut user_input = String::new();
    let (stdin, mut stdout) = (io::stdin(), io::stdout());
    
    
    loop {
        print!("\n\nEnter your command: ");
        
        user_input.clear();
        let _ = stdout.flush();
        let _ = stdin.read_line(&mut user_input)
            .map_err(|_| utils::print_usage_and_exit);
        
        let cmds = utils::parse_commands_and_quote_values(&user_input);
        let cmd_ref = cmds.iter()
            .map(String::as_str)
            .collect::<Vec<&str>>();

        match cmd_ref.as_slice() {
            ["list"] => {
                println!("Employees:");
                for emp in company.get_all_employees() {
                    println!("\t- {}", emp.name);
                }
            }
            ["list", "by", "department"] => {
                for (dep, emp_list) in company.get_all_by_department() {
                    println!("Department: {}", dep.name);
                    println!("{:#?}", emp_list
                        .iter()
                        .map(|e| e.name.as_str())
                        .collect::<Vec<&str>>());
                }
            }
            ["add", name, "to", dep] => {
                company.add_employee(name, dep);
                println!("Employee added!");
            }
            ["exit"] => std::process::exit(0),
            _ => utils::print_usage_and_exit(),
        }
    }
}

