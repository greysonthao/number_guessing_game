use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("\nLet's play the Number Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("\nA random number has been generated.");

    let mut counter = 0;

    loop {
        println!("\nPlease input your guess between 1 - 100.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failure to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("\n{}", "Too low.".red());
                counter += 1;
            }
            Ordering::Greater => {
                println!("\n{}", "Too high.".red());
                counter += 1;
            }
            Ordering::Equal => {
                println!("\n{}", "You're correct. You win!".green());
                counter += 1;
                println!("\nIt took you {} guesses.\n", counter);
                break;
            }
        }
    }
}
