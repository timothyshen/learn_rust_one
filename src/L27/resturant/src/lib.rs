

pub mod front_of_house {

    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
        fn complain() {}
    }
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::hosting::seat_at_table();
        crate::front_of_house::hosting::add_to_waitlist();
    }

    pub fn cook_order() {}
}


pub fn eat_at_resturant() {
    crate::front_of_house::hosting::add_to_waitlist();
    crate::front_of_house::hosting::seat_at_table();
}

//in lib.rs

