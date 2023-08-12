#![allow(non_snake_case)]
#![allow(unused)]

fn main() {
    println!("hello");
    my_function(3, 'h');
    let res = valid_function();
    println!("{}", res);
    let res2 = valid_function2(3, 5);
    println!("res2: {}" ,res2);

    // inline comment
    /*
        block comment
     */

    // control flow 
    // if else-if else
    let num = 3;
    if num < 5 {
        println!("condition true");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    }
    else {
        println!("condition false");
    }

    let cond = true;
    let num2 = if cond { 5 } else { 6 };
    println!("num2: {}", num2);

    // loop
    let mut counter = 0;
    let res = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
    };
    println!("counter: {}", counter);

    // while
    let mut num3 = 3;
    while num3 != 0 {
        println!("num3: {}", num3);
        num3 -= 1;
    }

    // for in
    let arr = [1, 2, 3];
    for element in arr {
        println!("the value arr is: {}", element);
    }
    
    for number in 1..=5 {
        println!("{}", number);
    }
    println!("bye..");

}

fn my_function(value: i32, label: char) {
    println!("Result: {}{}", value, label);
}

fn valid_function() -> i32 {
    5
}

fn valid_function2(x: i32, y: i32) -> i32 {
    x + y
}