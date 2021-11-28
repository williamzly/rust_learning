use std::collections::HashMap;
use std::io;

// Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.
fn average_of (list: &Vec<usize>) -> f32 {
    let mut total = 0;
    for i in list.iter() {
        total += i
    }
    let total = total as f32;
    total / (list.len() as f32)
}

fn median_of (list: &Vec<usize>) -> usize {
    let mut sorted: Vec<usize> = Vec::new();
    for i in 0..list.len() - 1 {
        sorted.push(list[i]);
    }
    for i in 0..sorted.len() {
        for j in (i+1)..sorted.len() {
            if sorted[i] > sorted[j] {
                let temp = sorted[i];
                sorted[i] = sorted[j];
                sorted[j] = temp;
            }
        }
    }
    sorted[sorted.len() / 2]
}

fn mode_of (list: &Vec<usize>) -> Vec<usize> {
    let mut counter: HashMap<usize, usize> = HashMap::new();
    for i in list.iter() {
        let count = counter.entry(*i).or_insert(0);
        *count += 1;
    }
    let mut mode: Vec<usize> = Vec::new();
    let mut max_count: usize = 0;
    for (num, count) in counter.iter() {
        if count > &max_count {
            max_count = *count;
            mode = vec!(*num);
        } else if count == &max_count {
            mode.push(*num);
        }
    }
    mode
}


// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn pig_latin(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }
    let vowel :[char; 5] = ['a', 'o', 'e', 'i', 'u'];
    let s = String::from(word);
    let chars = s.char_indices();
    let mut new_chars: Vec<char> = Vec::new();
    let mut is_start_with_vowel = false;
    let mut first_char: char = '0';
    for (i,  c) in chars {
        if i == 0 {
            if vowel.contains(&c) {
                is_start_with_vowel = true;
                new_chars.push(c);
            }
            first_char = c;
        } else {
            new_chars.push(c);
        }
    }
    let suffix = if is_start_with_vowel {
        String::from("-hay")
    } else {
        "-" .to_owned()+ &first_char.to_string() + "ay"
    };
    let mut result = String::new();
    for c in new_chars {
        result.push(c);
    }
    result + &suffix
}


// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

fn parse_user_input_for_adding_emploee(input: &str) -> Option<(&str, &str)> {
    let mut words = input.trim().split_whitespace();
    let maybe_add = words.next();
    if maybe_add.is_none()|| !maybe_add.expect("").eq_ignore_ascii_case("add") {
       return None;
    }
    let maybe_employee = words.next();
    if maybe_employee.is_none() {
        return None;
    }
    let maybe_to = words.next();
    if maybe_to.is_none()|| !maybe_to.expect("").eq_ignore_ascii_case("to") {
        return None;
    }
    let maybe_department = words.next();
    if maybe_department.is_none() {
        return None;
    } else {
        return Some((maybe_employee.expect(""), maybe_department.expect("")));
    }
}

fn employee_management_system_text_interface() {
    let mut company = Company::new();
    loop {
        let mut input = String::new();
        println!("Please input \n1 for add new employee \n2 for retrieve all people in a department \n3 for retrieve all people in company by department");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse() {
            Ok(i) => match i {
                1 => {
                    company.add_new_employee();
                    continue;
                },
                2 => {
                    company.retrieve_by_department();
                    continue;
                },
                3 => {
                    company.retrieve_all_people();
                    break;
                },
                _ => {
                    println!("nothing was matched");
                    continue;
                },
            },
            Err(_) => {
                println!("Err while parse input to u32");
                continue;
            }
        };
    }
}

struct Company {
    structure: HashMap<String, Vec<String>>
}

impl Company {
    fn new() -> Company {
        Company {
            structure: HashMap::new()
        }
    }
    fn add_new_employee(&mut self) {
        println!("Please input department and employee name as \"department employee\" (no quotes)");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let mut input = input.split_whitespace();
        let department = input.next().expect("");
        let employee = input.next().expect("");
        self.add_new_employee_internal(&department, &employee);
        println!("Added {} to {} success!", employee, department);
    }
    fn add_new_employee_internal(&mut self, department: &str, employee: &str) {
        match self.structure.get_mut(department) {
            Some(employees) => employees.push(employee.to_string()),
            None => {
                let mut employees = Vec::new();
                employees.push(String::from(employee));
                self.structure.insert(department.to_string(), employees);
            }
        }
    }
    fn retrieve_by_department(&mut self) {
        println!("Please input department name");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let maybe_employees = self.retrieve_by_department_internal(&input.trim());
        if maybe_employees.is_none() {
            println!("No employee in {}", input);
        } else {
            println!("---LIST---");
            for e in maybe_employees.expect("No employee in this department!!").iter() {
                println!("{}", e);
            }
            println!("----------");
        }
    }
    fn retrieve_by_department_internal(&mut self, department: &str) -> Option<&Vec<String>> {
        self.structure.get(department)
    }
    fn retrieve_all_people(&self) {
        let mut sorted = Vec::new();
        for employees in self.structure.values() {
            for e in employees {
                sorted.push(e);
            }
        }
        sorted.sort();
        println!("---LIST---");
        for e in sorted.iter() {
            println!("{}", e);
        }
        println!("----------");
    }
}


fn main() {
    let a_list = vec![2,0,2,1,1,1,1,4,2,2,3,3];
    println!("We have a list with {:?}", a_list);
    println!("It's average: {}", average_of(&a_list));
    println!("It's median: {}", median_of(&a_list));
    println!("It's mode: {:?}", mode_of(&a_list));

    println!("Apple's Pig Latin is {}", pig_latin("apple"));
    println!("World's Pig Latin is {}", pig_latin("world"));
    println!("empty Pig Latin is {}", pig_latin(""));
    println!("H's Pig Latin is {}", pig_latin("H"));

    
    println!("{:?}", parse_user_input_for_adding_emploee("aDd 123 to 321"));
    employee_management_system_text_interface();
}
