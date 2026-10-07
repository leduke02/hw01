use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::io;
use std::io::Write;

const START_GOLD: u64 = 100;
const FOOD_PRICE: u64 = 5;
const MAX_SEED: u64 = 9999;fn main() {

}
// Prints the five menu options
fn print_menu() {
    println!();
    println!("1. Buy food");
    println!("2. Hunt");
    println!("3. Trade with caravan");
    println!("4. View status");
    println!("5. Quit");
}
