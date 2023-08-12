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

pub fn main(){
    let head1 = value_in_cents(Coin::Head);
    println!("head1: {}", head1);
    let indo = value_in_cents(Coin::Flag(Asia::Indonesia));
    println!("{}", indo);
    let malay = value_in_cents(Coin::Flag(Asia::Malaysia));
    println!("{}", malay);
    let singapore = value_in_cents(Coin::Flag(Asia::Singapura));
    println!("{}", singapore);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Head => 1,
        Coin::Flag(country) => {
            print!("Country from {:?} is: ", country);
            5
        }
    }
}