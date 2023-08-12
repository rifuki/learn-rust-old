pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub content: String,
    pub location: String,
    pub author: String   
}

pub struct Tweet {
    pub content: String,
    pub username: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for NewsArticle {}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.content, self.username)
    }
}