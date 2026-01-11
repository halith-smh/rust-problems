fn main(){
    let mut greetings = String::from("Good Morning!");
    let s1 = &mut greetings;
    let s2 = &greetings;
    let s3 = &greetings;


    s1.push_str(" Halith");
}


// The above code will give error as:
// error[E0502]: cannot borrow `greetings` as immutable because it is also borrowed as mutable