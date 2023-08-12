#[allow(unused)]
enum Coin {
    Head,
    Flag(Asia)
}

#[derive(Debug)]
enum Asia {
    Indonesia,
    Malaysia,
    Singapura
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Head => 1,
        Coin::Flag(country) => {
            match country {
                Asia::Indonesia => 45,
                Asia::Malaysia => 57, 
                Asia::Singapura => 65,
            }
        }
    }
}

fn main() {
    let head1 = value_in_cents(Coin::Head);
    println!("head1: {}", head1);

    let flag_asia_1 = value_in_cents(Coin::Flag(Asia::Indonesia));
    println!("flag_asia_1: {}", flag_asia_1);
    let flag_asia_2 = value_in_cents(Coin::Flag(Asia::Malaysia));
    println!("flag_asia_2: {}", flag_asia_2);
    let flag_asia_3 = value_in_cents(Coin::Flag(Asia::Singapura));
    println!("flag_asia_3: {}", flag_asia_3);

}
