use std::collections::HashMap;

fn group_values_by_pair(users: Vec<(String, i32)>) -> HashMap<String, i32>{
    let mut result = HashMap::new();
    for (name, age) in users{
        result.insert(name, age);
    }

    result
}

fn main(){
    let users = vec![(String::from("Mohamed Halith"), 21), (String::from("Jon Snow"), 35)];
    let result = group_values_by_pair(users);

    println!("{:?}", result);
}