#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        return Point {
            x: self.x,
            y: other.y
        };
    }
}

fn main(){
    let point1 = Point {
        x: 2.5,
        y: true
    };
    let point2 = Point {
        x: 'c',
        y: "bacot"
    };

    let point3 = point1.mixup(point2);
    println!("point3.x is: {}, point3.y is: {}", point3.x, point3.y);

}