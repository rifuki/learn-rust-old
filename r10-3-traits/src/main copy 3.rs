use r10_3_traits::{Summary, Tweet};
fn main(){
    let tweet1 = Tweet {
        username: String::from("rifuki"),
        content: String::from("lorem ipsum dolor sit amet consectetur adispicing elit"),
        reply: false,
        retweet: false
    };
    println!("tweet 1: {}", tweet1.summarize());
}