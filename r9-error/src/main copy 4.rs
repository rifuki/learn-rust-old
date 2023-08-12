use std::io::{Read};
use std::fs::File;
pub fn main(){
    if let Ok(kotoba) = read_text_from_file() {
        println!("{}", kotoba);
    }
}

fn read_text_from_file() -> Result<String, std::io::Error> {
    let mut kotoba = String::new();

    File::open("hello.txt")?.read_to_string(&mut kotoba)?;
    Ok(kotoba)
}