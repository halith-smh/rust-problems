enum Size {
    S,
    M,
    L,
    XL,
    XXL,
}

fn main() {
    let user_size = Size::S;
    let num = decide_num(user_size);
    println!("{}", num);
}

fn decide_num(size: Size) -> &'static str {
    match size {
        Size::S => "30",
        Size::M => "32",
        Size::L => "36",
        Size::XL => "40",
        Size::XXL => "44",
    }
}
