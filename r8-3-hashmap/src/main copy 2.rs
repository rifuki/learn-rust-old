pub fn main() {
    let mut fav_chara = std::collections::HashMap::new();

    fav_chara.insert(String::from("ichiban"), String::from("Hatsune Miku"));
    fav_chara.insert(String::from("nibanme"), String::from("Kurumi Tokisaki"));
    fav_chara.insert(
        String::from("sanbanme"),
        String::from("Momo Belia Deviluke"),
    );
    fav_chara.insert(String::from("yonbanme"), String::from("Itsuka Kotori"));
    fav_chara.insert(String::from("gobanme"), String::from("Origami Toibichi"));

    let ichiban = String::from("ichiban");
    let who_is = fav_chara.get(&ichiban);
    println!("is: {}", if let Some(val) = who_is { val } else { "empty" });

    for (num, chara) in &fav_chara {
        println!("{} is: {}", num, chara);
    }
    println!("{:?}", fav_chara);

    fav_chara.insert("ichiban".to_string(), "Shiina Mashiro".to_owned());

    if let Some(chara) = fav_chara.get("ichiban") {
        println!("{}", chara);
    } else {
        println!("missing");
    }

    println!("fav_chara is: {:?}", fav_chara);
}