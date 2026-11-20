struct Employee {
    emp_id: u64,
    name: String,
    designation : String,
    fixed_salary: u32,
    variable_salary: u32
    // active : bool
}

impl Employee {
    fn total_salary(&self) -> u32 {
        self.fixed_salary + self.variable_salary
    }

    fn pf_deduction(&self, percent_cut: u32) -> u32 {
        let deduction = (percent_cut * self.total_salary())/100;
        return deduction;

    } 
}

fn main() {
    let employee = Employee {
        emp_id : 0,
        name :String::from("suraj kumar"),
        designation : String::from("Software Engineer"),
        fixed_salary : 12000,
        variable_salary: 80000
    };
    println!("the details employee is {:?} , {:?}, {:?}, with totalsalary of {:?} and deduction of {:?} ", employee.name, employee.emp_id, employee.designation, employee.total_salary(), employee.pf_deduction(30));
}
