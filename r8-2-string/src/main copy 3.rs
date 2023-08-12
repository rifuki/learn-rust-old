fn main(){
    let kotoba1 = String::from("sekai");
    let kotoba2 = "de".to_string();
    let kotoba3 = "ichiban".to_owned();

    let kotoba = format!("{}-{}-{}", kotoba1, kotoba2, kotoba3);
    println!("kotoba: {}", kotoba);
}