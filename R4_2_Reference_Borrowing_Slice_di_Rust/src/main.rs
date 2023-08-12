#![allow(non_snake_case)]
#![allow(unused)]

fn main() {
    let mut s1 = String::from("hello");
    change(&mut s1);
    let length = calculate_length(&s1);
    println!("the length of {} is {}", s1, length);

    // only one mutable reference at a same time
    let mut aozora = String::from("aozora");
    let r1 = &aozora;
    let r2 = &aozora;
    println!("r1: {}, r2: {}", r1, r2);
    let r3 = &mut aozora;
    r3.push_str("hello");

    // // dangle pointer
    // let reference_to_dangle = dangle();

    // slice 
    let hello_world = String::from("hello world");
    let first_word = first_word(&hello_world);
    println!("{}", first_word);
}

fn change(some_string: &mut String) {
    some_string.push_str(" world");
}

fn calculate_length(some_string: &String) -> usize {
    some_string.len()
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // s is dropped

fn first_word(some_string: &str) -> &str {
    let haha = some_string;
    let bytes = some_string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return some_string[..=i].as_ref();
        }
    }
    std::str::from_utf8(bytes).unwrap()
}