fn main() {
    let mut pinocchiop = std::collections::HashMap::new();
    pinocchiop.insert(String::from("Love for Love by Love of Love"), 2019);
    pinocchiop.insert("SlowMotion".to_owned(), 2014);

    for (key, val) in pinocchiop {
        println!("song: {}", key);
        println!("release date: {}", val);
        println!();
    }
}
