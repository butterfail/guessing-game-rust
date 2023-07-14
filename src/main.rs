extern crate termion;

use termion::{color, style};

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!(
        "{}Guess the number!{}",
        color::Fg(color::Blue),
        style::Reset
    );

    let min_range: u32 = 1;
    let max_range: u32 = 10;
    let secret_number = rand::thread_rng().gen_range(min_range..=max_range);
    let mut attempts: u32 = 0;
    let max_attempts: u32 = 3;

    loop {
        let remaining_attempts: u32 = max_attempts - attempts;
        let mut suffix = "";

        if remaining_attempts > 1 {
            suffix = "s";
        }

        println!(
            "{}You have {}{remaining_attempts}{} remaining attempt{suffix}{}",
            color::Fg(color::Blue),
            color::Fg(color::White),
            color::Fg(color::Blue),
            style::Reset
        );

        if attempts == max_attempts {
            println!("{}You lost!{}", color::Fg(color::Red), style::Reset);
            break;
        }

        println!(
            "{}Please pick a number between {}{min_range}{} and {}{max_range}{}.{}",
            color::Fg(color::Blue),
            color::Fg(color::White),
            color::Fg(color::Blue),
            color::Fg(color::White),
            color::Fg(color::Blue),
            style::Reset
        );

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect(
            "{}Failed to read line{}",
            color::Fg(color::Red),
            style::Reset,
        );

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!(
            "{}You guessed: {}{guess}{}",
            color::Fg(color::Blue),
            color::Fg(color::White),
            style::Reset
        );

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}Too small{}", color::Fg(color::Yellow), style::Reset),
            Ordering::Greater => println!("{}Too big{}", color::Fg(color::Red), style::Reset),
            Ordering::Equal => {
                println!("{}You win!{}", color::Fg(color::Green), style::Reset);
                break;
            }
        }

        attempts += 1;
    }
}
