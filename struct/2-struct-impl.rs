struct Rectangle {
    l: u32,
    b: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.l * self.b
    }

    fn perimeter(&self) -> u32 {
        2 * (self.l + self.b)
    }
}

fn main() {
    let rectangle_1 = Rectangle { l: 8, b: 4 };

    println!("Area of the rectangle: {}", rectangle_1.area());
    println!("Perimeter of the rectangle: {}", rectangle_1.perimeter());
}
