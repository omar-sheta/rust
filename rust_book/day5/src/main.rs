use rand::Rng;
use restaurant;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=101);
    restaurant::print_random_number(secret_number);
    println!("Hello, world!");
}
