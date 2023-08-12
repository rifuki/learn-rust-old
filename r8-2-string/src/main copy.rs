fn main() {
    let str1 = String::from("initial data");
    let str2 = "initial data".to_string();
    let str3 = "initial data".to_owned();

    println!("str1: {}", str1);
    println!("str2: {}", str2);
    println!("str3: {}", str3);
}
