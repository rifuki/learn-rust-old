fn main() {
    let v1 = vec![5, 0, 1, 2, 9];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    if total == 17 {
        println!("sama");
    }

}