fn main(){
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    filter_even(&mut nums);

    println!("{:?}", nums);
}

fn filter_even(nums: &mut Vec<i32>){
    let mut i = 0;

    while i < nums.len() {
        if nums[i] % 2 != 0{
            nums.remove(i);
        } else {
            i += 1;
        }
    }
}