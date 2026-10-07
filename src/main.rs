use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::io;
use std::io::Write;

const START_GOLD: u64 = 100;
const FOOD_PRICE: u64 = 5;
const MAX_SEED: u64 = 9999;


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
// Rolls count dice using seeds seed, seed + 1, ... and returns the sum
fn roll_many(seed: u64, count: u64) -> u64 {
    let mut sum = 0;
    for i in 0..count {
        sum += roll_die(seed + i);
    }
    sum
}
// Returns the food gained for a hunting roll
fn hunt_food(roll: u64) -> u64 {
    if roll == 1 {
        0
    } else if roll <= 3 {
        10
    } else if roll <= 5 {
        20
    } else {
        40
    }
}
// Returns true if gold is at least cost
fn can_afford(gold: u64, cost: u64) -> bool {
    gold >= cost
}
// Prints gold, food, and day
fn print_status(gold: u64, food: u64, day: u64) {
    println!("Day: {} | Gold: {} | Food: {}", day, gold, food);
}
fn main() {
    println!("Enter a seed number:");
    let seed = read_number(1, MAX_SEED);

    let mut gold = START_GOLD;
    let mut food = 0;
    let mut day = 1;
    let mut roll_count = 0;

    loop {
        print_menu();
        let choice = read_number(1, 5);

        if choice == 1 {
            // Buy food
            if !can_afford(gold, FOOD_PRICE) {
                println!("You cannot afford any food.");
            } else {
                let most = gold / FOOD_PRICE;
                println!("How many units of food?");
                let units = read_number(1, most);
                gold -= units * FOOD_PRICE;
                food += units;
                day += 1;
            }
        } else if choice == 2 {
            // Hunt
            let roll = roll_die(seed + roll_count);
            roll_count += 1;
            let gained = hunt_food(roll);
            food += gained;
            day += 1;
            println!("You rolled a {} and gained {} food.", roll, gained);
        } else if choice == 3 {
            // Trade with caravan
            let total = roll_many(seed + roll_count, 3);
            roll_count += 3;
            let earned = 2 * total;
            gold += earned;
            day += 1;
            println!(
                "You rolled a total of {} and earned {} gold.",
                total, earned
            );
        } else if choice == 4 {
            // View status
            print_status(gold, food, day);
        } else {
            // Quit
            println!("Final status:");
            print_status(gold, food, day);
            break;
        }
    }
}
