/**
 * Generate a random percentage for changing up a math problem
 * Edward Naidoo
 * Dec 4, 2022
 */

use rand::Rng;
use colored::*;
use round::round_up;

fn main() {
    // Generate random number with "%"
    let percent = rand::thread_rng().gen_range(1.0..16.0);
    
    // Rounds float to two decimal places
    println!("\n{}%", round_up(percent, 2));
    println!("{}", "Written by Edward Naidoo".yellow().bold());
}