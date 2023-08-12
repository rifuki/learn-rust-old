use r10_3_traits::{Summary, NewsArticle, Tweet};

fn main(){
    let tweet1 = Tweet {
        username: String::from("rifuki"),
        content: String::from("lorem ipsum dolor sit amet consectetur adipiscing elit"),
        reply: false,
        retweet: false
    };

    println!("tweet1: {}", tweet1.summarize());

    let news_article1 = NewsArticle {
        headline: String::from("new way to tweet"),
        location: String::from("jakarta"),
        author: String::from("aozora"),
        content: String::from("lorem ipsum dolor sit amet consectetur adipiscing elit")
    };

    println!("news_article1: {}", news_article1.summarize());
}
