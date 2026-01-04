struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user_1 = User {
        active: true,
        username: String::from("halith-smh"),
        email: String::from("sample@gmail.com"),
        sign_in_count: 1,
    };

    println!(
        "User Name: {}, The user is : {}",
        user_1.username,
        verify_user_active(user_1.active)
    );
}

fn verify_user_active(active_status: bool) -> &'static str {
    if active_status {
        return "Active";
    } else {
        return "In Active";
    }
}
