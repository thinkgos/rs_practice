pub mod hosting {
    pub fn add_to_waitlist() {
        println!("add_to_waitlist");
    }

    fn seat_at_table() {}
}

pub mod serving {
    pub fn take_order() {
        super::hosting::add_to_waitlist();
        println!("take_order");
    }

    pub fn server_order() {}

    fn take_payment() {}
}
