struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    fn calculate_area(self: &Self) -> u32 {
        self.length * self.width
    }

    fn width(self: &Self) -> bool {
        self.width > 0
    }

    fn can_hold(self: &Self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.length
    }
}

impl Rectangle {
    fn build(length: u32, width: u32) -> Rectangle {
        Rectangle {
            length,
            width
        }       
    }
}

fn main() {
    let rect1 = Rectangle { length: 10, width: 4 };
    println!("the area of rect1 is {} square pixel", rect1.calculate_area());
    let rect2 = Rectangle::build(12, 4);
    println!("the area of rect2 is {} square pixel", rect2.calculate_area());
    let rect3 = Rectangle::build(14, 3);
    println!("the area of rect3 is {} square pixel", rect3.calculate_area());
    println!("is rect3 has nonzero value: {}", rect3.width());

    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3: {}", rect1.can_hold(&rect3));
    println!("rect2 can hold rect3: {}", rect3.can_hold(&rect3));
}
