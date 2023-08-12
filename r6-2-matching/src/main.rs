#[allow(unused)]
#[derive(Debug)]
enum Coin {
    Head,
    Flag(Asia)
}

#[derive(Debug)]
#[allow(unused)]
enum Asia {
    Indonesia,
    Malaysia,
    Singapura
}

#[allow(unused)]
fn value_in_cents(coin: Coin) -> u8 {
    let mut count = 1;
    if let Coin::Flag(state) = coin {
        print!("the location is: {:?}", state);
        if let Asia::Indonesia = state {
            count * 10
        } 
        else {
            if let Asia::Malaysia = state {
                count * 2
            }
            else {
                count * 5
            }
        }
    } else {
        count 
    }
}

fn main(){
    let coin_head = value_in_cents(Coin::Head);
    println!("coin head: {}", coin_head);

    let coin_flag_indo = value_in_cents(Coin::Flag(Asia::Indonesia));
    println!(" coin_flag_indo: {}", coin_flag_indo);

    let coin_flag_malay = value_in_cents(Coin::Flag(Asia::Malaysia));
    println!(" coin_flag_malay: {}", coin_flag_malay);
    
    let coin_flag_singa = value_in_cents(Coin::Flag(Asia::Singapura));
    println!(" coin_flag_singa: {}", coin_flag_singa);
}