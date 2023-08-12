use rand::Rng;
fn main() {
    loop {
        let dice_number = rand::thread_rng().gen_range(1..=6);
        println!("dice: {}", dice_number);

        match dice_number {
            // 3 => {
            //     go_to_jail();
            //     break;
            // }
            // 4 => {
            //     buy_ticket();
            //     break;
            // }
            10 => {
                win();
                break;
            }
            _ => reroll(),
        }
    }
}

fn go_to_jail() {
    println!("go_to_jail!");
}

fn buy_ticket() {
    println!("buy_ticket!");
}

fn win() {
    println!("you win");
}
fn reroll() {
    println!("reroll");
}
