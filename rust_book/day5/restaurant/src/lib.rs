mod front_of_house { // module declaration
    // 
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// mod is by default private
// pub keyword makes it public
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// think of mod as a folder and fn as a file in that folder
// module tree
// crate
// └── front_of_house
//     ├── hosting
//     │   ├── add_to_waitlist
//     │   └── seat_at_table
//     └── serving
//         ├── take_order
//         ├── serve_order
//         └── take_payment


fn serve_order() {}

use crate::front_of_house::hosting; // use keyword to bring module into scope
// or use crate::front_of_house::hosting::add_to_waitlist;
// or self::front_of_house::hosting::add_to_waitlist;
mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        super::serve_order(); // super keyword to access parent module
    } 
    fn cook_order() {}

}