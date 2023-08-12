#![allow(non_snake_case)]

fn main() {
    let s = String::from("hello"); // s comes into scope
    take_ownership(s); // s value moved into the function
                       // s is not longer valid
    let x = 5; // x comes into scope
    makes_copy(x); // x value is copied
                   // use x afterward   
    // x goes out of scope, then s
}
fn take_ownership(some_string: String) {
    println!("{}", some_string);
} // here, some_string goes out of scope, and drop method is called

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // here, some_integer goes out of scope


