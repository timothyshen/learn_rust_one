
mod front_of_house;
mod back_of_house;



pub fn eat_at_resturant() {
    front_of_house::hosting::add_to_waitlist();
    crate::front_of_house::hosting::seat_at_table();
}

//in lib.rs

