pub fn add_two(num: i32) -> i32 {
    num + 2
}

pub fn prints_and_returns_100(num: i32) -> i32 {
    println!("i got the value: {}", num);
    100
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn this_test_will_pass(){
        let value = super::prints_and_returns_100(4);
        assert_eq!(value, 100);
    }
    
    #[test]
    #[ignore]
    fn this_test_will_fail(){
        let value = super::prints_and_returns_100(5);
        assert_eq!(value, 5);
    }

    #[test]
    #[ignore]
    fn add_two_and_two() {
        assert_eq!(4, super::add_two(2));
    }

    #[test]
    #[ignore]
    fn add_three_and_two() {
        assert_eq!(5, super::add_two(3));
    }

    #[test]
    #[ignore]
    fn one_hundred() {
        assert_eq!(102, super::add_two(100));
    }
}