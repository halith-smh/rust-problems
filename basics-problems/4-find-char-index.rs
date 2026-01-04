// Rust Program to find the char in a string at a index

fn main(){
    let greetings = String::from("Good Morning!, Halith");
    println!("{}", greetings); //macros in rust !

    let index = 12;

    let find_char = greetings.chars().nth(index);

    match find_char {
        Some(c) => println!("In index {}, the char is : {}", index, c),
        None => print!("No char at found at index: {}", index)
    };

}