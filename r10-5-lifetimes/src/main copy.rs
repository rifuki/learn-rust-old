fn main(){
    let kotoba1 = String::from("lorem ipsum dolor sit amet, ");
    let kotoba2 = "consectetur adipiscing elit.";

    println!("{}", find_longest_kotoba(&kotoba1, kotoba2));
}

fn find_longest_kotoba<'a>(kotoba1: &'a str, kotoba2: &'a str) -> &'a str {
    if kotoba1.len() > kotoba2.len() {
        kotoba1
    } else { 
        kotoba2
    }
}