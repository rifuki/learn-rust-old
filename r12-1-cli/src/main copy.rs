#[allow(unused)]
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file: {}", filename);

    let contents = std::fs::read_to_string(filename).expect("something went wrong reading the file");
    println!("\nwith text:\n{}", contents);
}
