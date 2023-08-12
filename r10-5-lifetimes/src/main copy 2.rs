#[allow(unused)]
struct ImportantExcerpt<'a> {
    part: &'a str
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        4
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("{}", announcement);
        self.part
    }
}

#[allow(unused)]
fn main(){
    let novel = String::from("lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do euismod tempor incididunt ut labore.");
    let first_word = novel.split('.').next().expect("could not find a '.'");

    let i = ImportantExcerpt {
        part: first_word
    };
    println!("{}", i.level());
    let part = i.announce_and_return_part("alert: lets get rusty");
}