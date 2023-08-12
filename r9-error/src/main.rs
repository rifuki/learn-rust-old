pub fn main(){
    if let Ok(kotoba) = read_text_from_file() {
        println!("hello.txt: {}", kotoba);
    }
}

fn read_text_from_file() -> Result<String, std::io::Error> {
    std::fs::read_to_string("hello.txt")
}