fn simulated_expensive_calculating(intensity: u32) -> u32 {
    println!("calculating slowly...");
    std::thread::sleep(std::time::Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!("today, do {} pushups!", simulated_expensive_calculating(intensity));
        println!("next, do {} situps!", simulated_expensive_calculating(intensity));
    } else {
        if random_number == 3 {
            println!("take a break today! remember to stay hydrated!");
        } else {
            println!("today, run for {} minutes", simulated_expensive_calculating(intensity));
        }
    }

}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
