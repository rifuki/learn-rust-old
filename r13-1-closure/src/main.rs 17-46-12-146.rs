use std::collections::HashMap;

fn create_multiplier(factor: i32) -> Box<dyn Fn(i32) -> i32> {
    let mut cache: HashMap<i32, i32> = HashMap::new();

    Box::new(move |number| {
        if let Some(&result) = cache.get(&number) {
            // Menggunakan cache jika sudah ada
            println!("Menggunakan cache...");
            return result;
        } else {
            println!("Menghitung...");
            let result = number * factor;
            cache.insert(number, result);
            return result;
        }
    })
}

fn main() {
    let multiply_by_two = create_multiplier(2);

    println!("{}", multiply_by_two(5)); // Output: Menghitung... 10
    println!("{}", multiply_by_two(5)); // Output: Menggunakan cache... 10
    println!("{}", multiply_by_two(3)); // Output: Menghitung... 6
    println!("{}", multiply_by_two(3)); // Output: Menggunakan cache... 6
}
