#![allow(non_snake_case)]
#![allow(unused)]

// regular struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple struct
struct Color(u32, u32, u32);

// unit strut
struct MyStruct;

fn main() {
    // create an instance
    let mut user1 = User {
        email: String::from("mahomarifuki@gmail.com"),
        username: String::from("rifuki"),
        active: true,
        sign_in_count: 1,
    };

    // access the fields of the struct instance
    println!("username: {}", user1.username);
    // change the field values of the struct instance
    user1.email = String::from("rikurifuky@gmail.com");
    println!("email: {}", user1.email);

    // create an object using a function that returns a struct instance
    let user2 = build_user(
        String::from("aozora"),
        String::from("aozoraumeko@gmail.com"),
    );

    // create an instance of another instance using the struct update syntax
    // moved value
    let user3 = User {
        username: user2.username,
        email: user1.email,
        active: user1.active,
        sign_in_count: user1.sign_in_count
    };
    // moved value
    let user4 = User {
        username: user1.username,
        ..user2
    };


    // instance tuple struct color
    let black = Color(0, 0, 0);

}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}
