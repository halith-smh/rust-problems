// Rust program to find out the given num is even

fn main() {
    let num = 23;
    if is_even(num){
        println!("{} is a even number", num);
    }else{
        println!("{} is not a even number (odd)", num);
    }
}

fn is_even(num: i32) -> bool {
    return num % 2 == 0;
}
