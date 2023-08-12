pub trait Summary {
    fn summarize(&self) -> String {
        format!("read more from ({})", self.summarize_author())
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
    pub author: String,
    pub headline: String,
    pub content: String,
    pub location: String
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub fn notify<T>(item: &T) -> String 
    where T: Summary 
{
    format!("breaking news: {}", item.summarize())
}

pub fn summarizable() -> impl Summary {
    Tweet {
        username: String::from("demo"),
        content: String::from("lorem ipsum dolor sit amet consectetur adipiscing elit sed do eismod tempor"),
        reply: false,
        retweet: false
    }
}
pub struct Pair<T, U> {
    pub x: T,
    pub y: U,
}

impl<T, U> Pair<T, U> {
    pub fn new(x: T, y: U) -> Self {
        Self { x, y }
    }
}

impl<T, U> Pair<T, U>
where
    T: std::fmt::Display + std::cmp::PartialOrd,
    U: std::fmt::Display + std::cmp::PartialOrd,
{
    pub fn cmp_display(&self) {
        if self.x > self.y {
            println!("the largest number is x = {}", self.x);
        } else {
            println!("the largest number is y = {}", self.y);
        }
    }
}
