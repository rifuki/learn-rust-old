use std::io::Read;
pub fn main(){
    if let Ok(kotoba) = read_text_from_file() {
        println!("hello.txt: {}", kotoba);
    }
}

fn read_text_from_file() -> Result<String, std::io::Error> {
    let mut kotoba = String::new();

    let mut f = std::fs::File::open("hello.txt")?;

    f.read_to_string(&mut kotoba)?;
    Ok(kotoba)
}