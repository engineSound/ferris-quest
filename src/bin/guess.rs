//! 🎲 Bonus game: Guess Ferris's Number!
//!
//! Run with `cargo run --bin guess`.
//! Once you've done a few lessons, read this file: you'll understand all of it!

use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let secret = random_number(1, 100);
    let mut tries = 0;

    println!("🦀 Ferris is thinking of a number between 1 and 100...");

    loop {
        print!("Your guess: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            println!("\n👋 Bye! The number was {secret}.");
            return;
        }

        let guess: u32 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("🤔 That's not a number! Try again.");
                continue;
            }
        };
        tries += 1;

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => println!("⬆️  Higher! {}", hint(secret, guess)),
            std::cmp::Ordering::Greater => println!("⬇️  Lower! {}", hint(secret, guess)),
            std::cmp::Ordering::Equal => {
                println!("🎉 YES! {secret} it is! You got it in {tries} tries.");
                println!("{}", rating(tries));
                return;
            }
        }
    }
}

fn hint(secret: u32, guess: u32) -> &'static str {
    match secret.abs_diff(guess) {
        0..=3 => "🔥🔥🔥 Super hot!",
        4..=10 => "🔥 Warm!",
        11..=25 => "🌤️  Lukewarm.",
        _ => "🧊 Brrr, cold.",
    }
}

fn rating(tries: u32) -> &'static str {
    match tries {
        1 => "🤯 First try?! Are you a mind-reading crab?",
        2..=5 => "🏆 Amazing! Ferris is impressed.",
        6..=8 => "😄 Nicely done!",
        _ => "🐢 Slow and steady wins the race!",
    }
}

/// A tiny "random" number from the clock, so we don't need any crates.
fn random_number(min: u32, max: u32) -> u32 {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
    min + nanos % (max - min + 1)
}
