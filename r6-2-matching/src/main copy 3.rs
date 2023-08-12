fn plus_one(x: Option<u32>) -> Option<u32> {
    match x {
        Some(i) => Some(i + 1),
        None => None
    }
}

fn main(){
    let satu = Some(1);
    let satu_plus = plus_one(satu);
    println!("satu_plus: {}", satu_plus.unwrap());

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six: {}", six.unwrap());
    println!("none: {}", none.expect("nonononononono"));
 
}
