use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Let's play the Number Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Your secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failure to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed {} which is too low.", &guess),
            Ordering::Greater => println!("You guessed {} which is too high.", &guess),
            Ordering::Equal => {
                println!("You guessed {} which is correct. You win!", &guess);
                break;
            }
        }
    }
}
