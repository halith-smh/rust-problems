fn main() {
    let full_name = String::from("Robb Stark");

    let fist_name = get_first_name(&full_name);
    println!("The first name of `{}` is: {}", full_name, fist_name);
}

fn get_first_name(full_name: &String) -> String {
    let mut first_name = String::new();

    for char in full_name.chars() {
        if char == ' ' {
            break;
        }
        first_name.push(char);
    }

    first_name
}