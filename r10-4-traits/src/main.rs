use r10_4_traits::{Summary, Tweet, NewsArticle, notify, summarizable};
fn main(){
    let tweet1 = Tweet {
        username: String::from("rifuki"),
        content: String::from("lorem ipsum dolor sit amet consectetur adipiscing elit sed do eismod tempor"),
        reply: false,
        retweet: false
    };
    let news1 = NewsArticle {
        author: String::from("iuchi"),
        content: String::from("lorem ipsum dolor sit amet consectetur adipiscing elit sed do eismod tempor"),
        location: String::from("jakarta"),
        headline: String::from("a new way to tweet")
    };
    println!("{}", tweet1.summarize());
    println!("{}", news1.summarize());

    println!("{}", notify(&tweet1));
    println!("{}", notify(&news1));

    let summarizable1 = summarizable();
    println!("summarizable1: {}", summarizable1.summarize());
    println!("summarizable1: {}", summarizable1.summarize_author());

    let newa1 = r10_4_traits::Pair::new(34, 4);
}