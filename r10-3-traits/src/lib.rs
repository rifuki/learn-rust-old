pub trait Summary {
    fn summarize(&self) -> String {
        format!("read more from ({})...", self.summarize_author())
    }
    
    fn summarize_author(&self) -> String;
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub retweet: bool,
    pub reply: bool
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub location: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary) -> String {
    format!("Breaking News! {}", item.summarize())
}