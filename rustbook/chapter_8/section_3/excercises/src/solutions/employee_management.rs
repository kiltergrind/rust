
use std::collections::HashMap;


type DepartmentCollection = HashMap<Department, Vec<Employee>>;


#[derive(Debug, PartialEq)]
pub struct Company {
    departments: DepartmentCollection,
}

impl Company {
    pub fn new() -> Self {
        Self { departments: Default::default(), }
    }

    pub fn add_employee(&mut self, employee: &str, department: &str) {
        self.departments
            .entry(Department::with_name(department))
            .or_default()
            .push(Employee::with_name(employee));
    }

    pub fn get_all_employees(&self) -> Vec<Employee> {
        let mut emp_list = vec![];
        for dep_emp_list in self.departments.values() {
            emp_list.extend_from_slice(dep_emp_list);
        }
        emp_list
    }

    pub fn get_all_by_department(&self) -> &DepartmentCollection {
        &self.departments
    }
}


#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Department {
    pub name: String,
}

impl Department {
    pub fn with_name(name: &str) -> Self {
        Department {
            name: name.to_owned(),
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct Employee {
    pub name: String,
}

impl Employee {
    pub fn with_name(name: &str) -> Self {
        Employee {
            name: name.to_owned(),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    
    /// Helper utils
    mod test_utils {
        use super::{Department, Company, Employee};

        pub type TestDepartments = Vec<(Department, Vec<Employee>)>;
        
        pub fn check_employees(
            company: &Company,
            expected: TestDepartments,
        ) -> bool
        {
            let departments = &company.departments;
            for (name, expected_names) in expected {
                if let Some(names) = departments.get(&name) {
                    assert_eq!(names, &expected_names);
                    continue;
                }
                return false;
            }
            true
        }

        pub fn create_sample_company() -> (Company, TestDepartments) {
            let mut company = Company::new();
            let test_departments: TestDepartments = vec![
                (
                    Department { name: "Engineering".to_owned() },
                    vec![
                        Employee { name: "John".to_owned() },
                        Employee { name: "Hannah".to_owned() },
                    ],
                ),
                (
                    Department { name: "DevOps".to_owned() },
                    vec![
                        Employee { name: "Alex".to_owned() },
                    ],
                ),
            ];
            company.add_employee("John", "Engineering");
            company.add_employee("Hannah", "Engineering");
            company.add_employee("Alex", "DevOps");
            
            (company, test_departments)
        }
    }


    #[test]
    fn test_add_employee() {
        let (company, departments)
            : (Company, test_utils::TestDepartments)
            = test_utils::create_sample_company();
        
        assert!(test_utils::check_employees(
            &company,
            departments,
        ));
    }

    #[test]
    fn test_get_all_employees() {
        let (company, mut departments)
            : (Company, test_utils::TestDepartments)
            = test_utils::create_sample_company();
        let mut expected = vec![];
        let employees = company.get_all_employees();

        for dep in departments.iter_mut() {
            expected.append(&mut dep.1);
        }

        assert!(employees.iter().all(|item| expected.contains(item)));
    }

    #[test]
    fn test_get_all_by_department() {
        let (company, mut departments)
            : (Company, test_utils::TestDepartments)
            = test_utils::create_sample_company();
        let mut employees = company.get_all_by_department().values().flatten();
        let mut expected = vec![];

        for dep in departments.iter_mut() {
            expected.append(&mut dep.1);
        }

        assert!(employees.all(|item| expected.contains(item)));
    }

}