pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub location: String,
    pub content: String
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return format!("{}, by {} ({})", self.headline, self.author, self.location);
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("{}, by {}", self.content, self.username);
    }
}
