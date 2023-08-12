#![allow(non_snake_case)]

fn main() {
    {
        // // s is not valid here, it's not yet declared
        // print!("{}", s);
        let s = "hello"; // s is valid from this point forward
        println!("{}", s);
    } // the scope is over, s is no longer valid

    let mut s = String::from("hello"); // s is string literal
    s.push_str(" world");
    println!("{}", s);

    let x = 5;
    let y = x; // copy value occurs
    println!("{}", y);

    let s1 = String::from("hello");
    let s2 = s1; // moved value
    // println!("s1: {}", s1);
    println!("s2: {}", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone(); // cloned value cause expensive memory 
    println!("s1: {}", s1);
    println!("s2: {}", s2);

}
