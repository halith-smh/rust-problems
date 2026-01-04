// Rust program to find out fibonacci of a number

fn main() {
    let num = 8;
    println!("{}", fib(num));
}

// 0, 1, 1, 2, 3, 5, 8, 13, .....

fn fib(num: i32) -> i32 {
    if num < 0 {
        panic!("Fibonacci is not defined for negative numbers");
    }

    if num == 0 {
        return 0;
    }

    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..num {
        let next = prev + curr;
        prev = curr;
        curr = next
    }

    return curr;
}
