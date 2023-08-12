#[cfg(test)]

fn greeting(name: &str) -> String {
    let mut s = String::from("ohayou, ");
    s.push_str(name);
    s
}

mod tests {
    #[test]
    fn greeting() {
        let result = super::greeting("aozora");
        assert!(result.contains("aozora"), "Greeting did not contain name, value was {}", result);
    }
}