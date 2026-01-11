 fn main(){
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = filter_even(nums);

    println!("{:?}", result);
}

// A fn() that will take input as a vector and return the filtered vector that conatins even numbers
fn filter_even(nums: Vec<i32>) -> Vec<i32>{
    let mut result = Vec::new();
    for num in nums{
        if num % 2 == 0{
            result.push(num);
        }
    }

    result
}