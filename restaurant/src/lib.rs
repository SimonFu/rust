mod front_of_house;

fn serve_order() {}

use front_of_house::hosting;

pub fn eat_in_restaurant() {
    hosting::add_to_wishlist();
    hosting::add_to_wishlist();
    front_of_house::fix_incorrect_order();
}
