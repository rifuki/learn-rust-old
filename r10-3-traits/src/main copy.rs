use r10_3_traits::{Summary, Tweet};

fn main() {
    let tweet1 = Tweet {
        username: String::from("rifuki"),
        content: String::from("lorem ipsum dolor sit amet consectetur adipiscing elit"),
        reply: false,
        retweet: true
    };
    println!("tweet1 is: {}", tweet1.summarize());
}
