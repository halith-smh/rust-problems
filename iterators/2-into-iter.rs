fn main(){
    let nums = vec![1, 2,3];
    let iter = nums.into_iter();

    for val in iter{
        println!("{}", val);
    }
}