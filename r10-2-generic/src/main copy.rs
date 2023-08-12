struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn get_x_value(self: &Self) -> &T {
        &self.x
    }

    fn get_y_value(self: &Self) -> &T {
        &self.y
    }
}

fn main(){
    let point1 = Point {
        x: 30,
        y: 12
    };
    println!("point1.x is: {}", point1.get_x_value());
    println!("point1.y is: {}", point1.get_y_value());
}
