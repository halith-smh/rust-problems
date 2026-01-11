use std::collections::HashMap;

struct Users {
    name: String,
    age: i32,
}

fn group_values_by_pair(users: Vec<Users>) -> HashMap<String, i32> {
    let mut result = HashMap::new();

    for user in users {
        result.insert(user.name, user.age);
    }

    result
}

fn main() {
    let users: Vec<Users> = vec![
        Users {
            name: String::from("Mohamed Halith"),
            age: 21,
        },
        Users {
            name: String::from("Jon Snow"),
            age: 35,
        },
    ];

    let result = group_values_by_pair(users);
    println!("{:?}", result);
}
