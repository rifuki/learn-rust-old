fn main(){
    let mut foo = String::from("foo ");
    foo.insert_str(foo.len(), "bar");
    println!("{}", foo);

    let mut chara = String::from("o");
    chara.push('k');
    println!("chara: {}", chara);

    let mut kotoba1 = String::from("kusottare");
    let kotoba2 = String::from(" ga");

    kotoba1.push_str(&kotoba2);

    let kotoba3 = kotoba1 + &kotoba2;
    println!("kotoba3: {}", kotoba3);
}