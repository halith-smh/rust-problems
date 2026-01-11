fn main() {
    let a1: String = String::from("Mohamed Halith");
    // The owner is a1 but the function &a1 is borrowing
    do_something(&a1);
    println!("{}", a1);
}

// Reference - The type of refered string into the function
fn do_something(a1: &String){
    println!("{}", a1)
}

