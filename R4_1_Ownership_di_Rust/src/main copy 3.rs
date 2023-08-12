#![allow(non_snake_case)]

fn main() {
    let s1 = gives_ownership();
    println!("{}", s1);
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // moved value s2
    println!("{}", s3);
}

fn gives_ownership() -> String {
    let some_str = String::from("world");
    some_str
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}