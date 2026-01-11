fn main() {
    let a1 = String::from("Mohamed Halith");
    let _a2 = a1;
    println!("{}", a1);
}

// The above code will produce error at line 4
// borrow of moved value: `a1`value borrowed here after move