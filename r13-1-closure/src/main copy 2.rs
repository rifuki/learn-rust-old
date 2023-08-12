fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure= | num | {
        println!("calculating slowly...");
        std::thread::sleep(std::time::Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("today, do {} pushups!", expensive_closure(intensity));
        println!("next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("take a break today! remember to stay hydrated!");
        } else {
            println!("today, run for {} minutes", expensive_closure(intensity));
        }
    }
}
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}