#[allow(unused)]
fn main(){
    let konichiwa = String::from("こんにちは");
    for i in konichiwa.chars(){
        print!("{}", i);
    }

    // let kotoba = "ichiban".to_string();
    // let first_byte = kotoba.as_bytes()[0];
    // println!("{}", std::str::from_utf8(&[first_byte]).unwrap());
}