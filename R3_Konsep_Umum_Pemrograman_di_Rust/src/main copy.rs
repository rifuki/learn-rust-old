#![allow(non_snake_case)]
#![allow(unused)]

fn main() {
    let a = 10; // immutable
    let mut b = 20; // mutable

    // constant
    const HIGHEST_PRICE: u32 = 100000;
    const HIGHEST_PRICE_2: u32 = 100_000;

    // shadowing 
    let x = 10; 
    let x = "ten";

    // scalar data type: integer, floating, boolean, character
    let i: u16 = 10;
    let f: f32 = 3.14;
    let valid = true;
    let valid: bool = false;
    let c = 'c';

    // compound data type: tuple, array
    let tup = (1, 1.3, true, 'c'); // tupple
    let (i, f, b, c) = tup;
    let first_tup = tup.0;
    let arr1 = [1, 2, 3]; // array
    let arr2: [i32; 5] = [1, 2, 3, 4, 5]; 
    let arr3 = [5u32; 3];
    let first_arr1 = arr1[1];
    let second_arr2 = arr2.get(2);

    // special data type : unit
    let unit = ();

}
