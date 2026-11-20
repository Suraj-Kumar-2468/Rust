fn get_string_length(stringveriable: String) -> usize {
    return stringveriable.chars().count();
}

fn main() {
    let stringvariable = String::from("Hello World");
    let length = get_string_length(stringvariable);
    println!("The length of given string is {}", length)    
}
