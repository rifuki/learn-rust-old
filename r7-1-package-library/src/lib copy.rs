mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){

        }
        pub fn seat_at_table(){

        }
    }
    mod serving {
        fn server_order(){}

        mod back_of_house {
            use super::server_order;
            fn incorrect_order(){
                cook_order();
                server_order()
            }
            fn cook_order() {}
        }
        // pub fn take_order(){}
        // pub fn serve_order(){}
        // pub fn take_payment(){}
    }
}

pub fn eat_at_restaurant(){
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}