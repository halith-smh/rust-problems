use std::collections::HashMap;

fn main(){
    let mut users: HashMap<String, i32> = HashMap::new();

    users.insert(String::from("Jon Snow"), 21);

    let user1 = users.get("Jon Snow");

    match user1 {
        Some(age) => println!("Age is: {}", age),
        None => println!("User not found")
    };
}

// HashMap Methods
// 1. insert
// 2. get
// 3. remove
// 4. clear