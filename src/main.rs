use rand::seq::SliceRandom;
use rand::rng;
use num2words::Num2Words;
use std::io::{self, Write};

fn main() {
    println!("📚 English Number Practice with Rust!");

    let (min, max) = get_range();

    let mut numbers: Vec<u32> = (min..=max).collect();
    let mut rng = rng();

    loop {
        if numbers.is_empty() {
            println!("\n🎉 You've practiced all numbers in the range!");
            break;
        }

        numbers.shuffle(&mut rng);
        let number = numbers.pop().unwrap(); // safe unwrap because we checked is_empty

        println!("\n🔢 Number: {number}");
        print!("✍️  Write the number in English (or type 'exit'): ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        let user_input = user_input.trim().to_lowercase();

        if user_input == "exit" {
            println!("👋 Goodbye!");
            break;
        }

        let correct_answer = Num2Words::new(number)
            .to_words()
            .unwrap_or_default()
            .to_lowercase();

        if user_input == correct_answer {
            println!("✅ Correct!");
        } else {
            println!("❌ Wrong. Correct: {}", correct_answer);
        }
    }
}

fn get_range() -> (u32, u32) {
    println!("Enter range for numbers to practice:");

    let min = loop {
        print!("🔽 Minimum: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(n) = input.trim().parse::<u32>() {
            break n;
        }
        println!("❗️ Please enter a valid number.");
    };

    let max = loop {
        print!("🔼 Maximum: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(n) = input.trim().parse::<u32>() {
            if n >= min {
                break n;
            }
        }
        println!("❗️ Please enter a valid number greater than or equal to {}", min);
    };

    (min, max)
}