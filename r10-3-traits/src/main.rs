use r10_3_traits::{Summary, NewsArticle, Tweet };
fn main(){
    let tweet1 = Tweet {
        username: String::from("rifuki"),
        content: String::from("lorem ipsum dolor sit amet consectetur adipiscing elit"),
        reply: false,
        retweet: false
    };
    println!("tweet1: {}", tweet1.summarize());
    println!("{}", r10_3_traits::notify(&tweet1));

    let news1 = NewsArticle {
        content: String::from("lorem ipsum dolor sit amet consectetur adipiscing elit"),
        author: String::from("aozora"),
        location: String::from("jakarta"),
        headline: String::from("new way to tweet")
    };
    println!("news1: {}", news1.summarize());
    println!("{}", r10_3_traits::notify(&news1));
    
    let tweet2 = Tweet {
        username: String::from("iuchi"),
        content: String::from("way way way iuchi"),
        reply: false,
        retweet: false
    };
}