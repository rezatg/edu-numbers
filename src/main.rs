use rand::{rng, seq::{IndexedRandom}};
use num2words::Num2Words;
use std::io::{self, Write};

fn main() {
    println!("📚 English Number Practice with Rust!");
    let (min, max) = get_range();
    let numbers: Vec<u32> = (min..=max).collect();
    practice_loop(numbers);
}

fn get_range() -> (u32, u32) {
    println!("Enter range for numbers to practice:");

    let min = read_number("🔽 Minimum: ");
    let max = loop {
        let n = read_number("🔼 Maximum: ");
        if n >= min {
            break n;
        }
        println!("❗️ Please enter a number ≥ {min}");
    };

    (min, max)
}

fn read_number(prompt: &str) -> u32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(n) = input.trim().parse::<u32>() {
            return n;
        }

        println!("❗️ Please enter a valid number.");
    }
}

fn practice_loop(mut numbers: Vec<u32>) {
    let mut rng = rng();

    while let Some(number) = numbers.choose(&mut rng).cloned() {
        numbers.retain(|&n| n != number); // remove the selected number

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

    println!("\n🎉 You've practiced all numbers in the range!");
}
