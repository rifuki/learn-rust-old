mod roles {
    pub(crate) mod front_of_house {
        pub(crate) mod hosting {
            pub(crate) fn greeting_guests() {}
            pub(crate) fn add_to_waitlist() {}
            pub(crate) fn seat_at_table() {}
        }
        pub(crate) mod serving {
            pub(crate) fn take_order() {}
            pub(crate) fn serve_order() {}
            pub(crate) fn take_payment() {}

            mod back_of_house {
                fn fix_incorrect_order() {
                    cook_order();
                    super::take_order();
                }
                fn cook_order() {}
            }

        }
    }
}