struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn get_x_value(self: &Self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(self: &Self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let point1 = Point {
        x: 10,
        y: 5
    };
    println!("point1.x is: {}", point1.get_x_value());

    let point2 = Point {
        x: 10.0,
        y: 5.0
    };
    println!("point2 distance_from_origin is: {}", point2.distance_from_origin());
}

