struct Guess {
    value: i32
}

impl Guess {
    fn new (value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 1, got {}.", value)
        }

        Guess { value }
    }
}

mod tests {
    #[test]
    #[should_panic(expected = "Guess value must be greater than or equal to 1")]
    fn guessing() {
        super::Guess::new(0);
    }
}