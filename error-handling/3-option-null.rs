// Implementation of None -> null pointer using --- Option Enum ---

fn find_index(input_string: String, target_char: char) -> Option<usize> {
    for (index, char) in input_string.chars().enumerate(){
         if char == target_char{
            return Some(index);
         }  
    }
    return None;
}

fn main(){
    let s1 = String::from("Mohamed Halith S");

    let result = find_index(s1, 'S');

    match result {
        Some(index) => println!("The Char S is found at index: {}", index),
        None => println!("There is no char S found at the string"),
    };

}