fn main(){
    let s = String::from("Jon Snow");
    println!("The length of the string is: {}", get_str_len(s));
}

fn get_str_len(s: String) -> usize {
    s.chars().count() //implicit return
}