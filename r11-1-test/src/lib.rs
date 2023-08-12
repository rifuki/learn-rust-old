mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 1 + 1 == 2 {
            Ok(())
        } else {
            Err(String::from("1 + 1 harusnya 2"))
        }
    }
}