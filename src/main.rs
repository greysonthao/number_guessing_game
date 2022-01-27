use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Let's play the Number Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failure to read line.");

        println!("Your guess is {}", guess);

        let guess: u32 = guess.trim().parse().expect("Please input a number.");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Your guess is too low.".red()),
            Ordering::Greater => println!("{}", "Your guess is too high.".red()),
            Ordering::Equal => {
                println!("{}", "Your guess is correct! You win!".green());
                break;
            }
        }
    }
}
