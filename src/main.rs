use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::io;
use std::io::Write;

const START_GOLD: u64 = 100;
const FOOD_PRICE: u64 = 5;
const MAX_SEED: u64 = 9999;
fn main() {

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
// Keeps asking until the user types a whole number from min to max
fn read_number(min: u64, max: u64) -> u64 {
    loop {
        print!("Enter a number from {} to {}: ", min, max);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let bytes = io::stdin().read_line(&mut input).unwrap();
        if bytes == 0 {
            // input was closed, so stop instead of asking forever
            std::process::exit(0);
        }
	
   let parsed = input.trim().parse::<u64>();
        match parsed {
            Ok(number) => {
                if number >= min && number <= max {
                    return number;
                }
                println!("That number is out of range.");
            }
            Err(_) => println!("That is not a whole number."),
   }
    }
}

   
// Returns a number from 1 to 6
fn roll_die(seed: u64) -> u64 {
    let mut rng = StdRng::seed_from_u64(seed);
    rng.random_range(1..=6)
}
