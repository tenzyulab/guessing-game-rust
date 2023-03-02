use rand::Rng;

use std::cmp::Ordering::*;

mod utils;
use utils::macros::*;

fn main() {
    println!("Guess the number!");
    println!("You have 7 tries to guess the number between 1 and 100.");

    let mut try_count = 0;
    let secret = rand::thread_rng().gen_range(1..=100);

    d_println!("The secret number is {}", secret);

    loop {
        println!("=================");
        println!("Please input your guess. (Try {}/7)", try_count + 1);

        let Ok(raw_guess) = utils::read_line() else {
            println!("[Error]: Error reading line");
            continue;
        };

        let guess = match raw_guess.trim().parse::<u32>() {
            Ok(guess @ 1..=100) => {
                try_count += 1;
                guess
            }
            Ok(_) => {
                println!("[Error]: Please type a number between 1 and 100!");
                continue;
            }
            Err(_) => {
                println!("[Error]: Please type a number!");
                continue;
            }
        };

        match guess.cmp(&secret) {
            Less => {
                println!("[Hint]: Too small!");
            }
            Greater => {
                println!("[Hint]: Too big!");
            }
            Equal => {
                println!("[Result]: You win!");
                println!("[Result]: You guessed in {try_count} tries!");
                break;
            }
        };

        if try_count == 7 {
            println!("[Result]: You lost!");
            break;
        }
    }
}
