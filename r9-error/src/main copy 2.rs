use std::io::Read;
pub fn main(){
    let mut temp_string = String::new();

    let f = std::fs::File::open("hello.txt");
    let mut huhu = match f {
        Ok(data) => data,
        Err(error) => panic!("{}", error)
    };
    huhu.read_to_string(&mut temp_string).expect("woilah");
    println!("read: {}", temp_string);
}