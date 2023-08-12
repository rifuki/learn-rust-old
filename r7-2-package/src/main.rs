mod my_modules {
    pub(crate) mod front_of_house {
        pub(crate) mod hosting {
            fn greet_guests(){}
            pub(crate) fn add_to_waitlist(){
                println!("add to waitlist")
            }
            fn seat_at_table(){}
        }
        mod serving {
            fn take_order(){}           
            fn serve_order(){}
            fn take_payment(){}
        }
    }

    pub(crate) use self::front_of_house::hosting;
    pub(crate) fn eat_at_restaurant() {
        println!("eat at restaurant");
        hosting::add_to_waitlist();
    }
}

fn main() {
    let add1 = self::my_modules::eat_at_restaurant();
    println!("in main");
    my_modules::hosting::add_to_waitlist()
}