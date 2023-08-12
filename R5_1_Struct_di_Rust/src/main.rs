#![allow(non_snake_case)]
#![allow(unused)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    // let width = 30;   
    // let height = 50;

    // let rect = (30, 50);

    let rect = Rectangle {
        width: 30,
        height: 50
    };


    // println!("The area of rectangle is {} square pixels", area(width, height));
    // println!("The area of rectangle is {} square pixels", area(rect));
    println!("The area of rectangle is {} square pixels", area(&rect));

    /* debugging struct */
    println!("rect: {:?}", rect);
    dbg!(rect);
}

// fn area(height: u32, width: u32) -> u32 {
//     height * width
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1 
// }

fn area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}