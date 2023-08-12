#![allow(non_snake_case)]
use rand::{thread_rng, Rng};
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    io::{stdin, stdout, Write},
};

fn main() {
    println!("Guess the number!");
    let secret_number = thread_rng().gen_range(1..=100);

    loop {
        print!("Please input your guess: ");
        stdout().flush().unwrap();

        let mut guess = String::new();
        stdin().read_line(&mut guess).unwrap();
        
        let guess = match guess.trim().parse::<u32>() {
            Ok(number) => number,
            Err(_) => {
                println!("Please input number!");
                continue;
            }
        };
        
        match guess.cmp(&secret_number) {
            Less => println!("Too Less"),
            Greater => println!("Too Greater"),
            Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
