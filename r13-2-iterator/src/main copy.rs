fn main() {
    let v1 = vec![2, 3, 5, 8, 10];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("element: {}", val);
    }
}
