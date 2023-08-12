struct Cacher<T>
where
      T: Fn(u32) -> u32
{
    closure: T,
    value: Option<u32>
}

impl<T> Cacher<T>
where
      T: Fn(u32) -> u32
{
    fn new(closure: T) -> Self {
        Self {closure, value: None}
    }

    fn call(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(val) => val,
            None => {
                let v = (self.closure)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        std::thread::sleep(std::time::Duration::from_secs(2));
        num
    });
    
    if intensity < 25 {
        println!("today, do {} pushups!", expensive_closure.call(intensity));
        println!("next, do {} situps!", expensive_closure.call(intensity));
    } else {
        if random_number == 3 {
            println!("take a break today! remember to stay hydrated!");
        } else {
            println!("today, runs for {} minutes", expensive_closure.call(intensity));
        }
    }
}

fn main() -> () {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}