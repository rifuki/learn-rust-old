fn main() {
    let mut v1: Vec<i64> = Vec::new();
    v1.push(-100);

    let v2 = Vec::from([1, 2, 3, 4, 5]);
    println!("{:?}", v2[2]);
    if let Some(data) = v2.get(7) {
        println!("v2-[3] is: {}", data);
    }
    match v2.get(5) {
        Some(data) => println!("data: {}", data),
        None => println!("hayoooo")
    }
}
