mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
    pub mod serving {
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn eat(toast: String) -> Breakfast {
            Breakfast { toast, seasonal_fruit: String::from("apple") }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad
    }

}

pub fn eat_at_restaurant(){
    let mut meal = crate::front_of_house::Breakfast::eat(String::from("Sari Roti"));
    meal.toast = "Borobudur".to_owned();
    println!("{}", meal.toast);

    let order1 = crate::front_of_house::Appetizer::Soup;
    let order2 = crate::front_of_house::Appetizer::Salad;
}