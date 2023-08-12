pub trait Summary {
    fn summarize(&self) -> String {
        format!("read more from {}...", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

pub struct NewsArticle {
    pub content: String,
    pub author: String,
    pub location: String,
    pub headline: String
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)       
    }
}