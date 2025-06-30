use rand::{seq::SliceRandom, rng};
use num2words::Num2Words;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::process;
use ctrlc::set_handler;

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
    let total = numbers.len();
    let correct = Arc::new(std::sync::Mutex::new(0));
    let wrong = Arc::new(std::sync::Mutex::new(0));
    let answered = Arc::new(std::sync::Mutex::new(0));

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let c_correct = Arc::clone(&correct);
    let c_wrong = Arc::clone(&wrong);
    let c_answered = Arc::clone(&answered);

    set_handler(move || {
        println!("\n🛑 Interrupted by user.");
        print_summary(*c_correct.lock().unwrap(), *c_wrong.lock().unwrap(), total, *c_answered.lock().unwrap());
        r.store(false, Ordering::SeqCst);
        process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    while !numbers.is_empty() && running.load(Ordering::SeqCst) {
        numbers.shuffle(&mut rng);
        let number = *numbers.last().unwrap();

        let progress = *answered.lock().unwrap();
        println!("\n🔢 Number: {number}    ({}/{})", progress + 1, total);

        print!("✍️  Write the number in English (or type 'exit'): ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        let result = io::stdin().read_line(&mut user_input);
        if result.is_err() {
            println!("❗ Error reading input. Try again.");
            continue;
        }

        let user_input = user_input.trim().to_lowercase();

        if user_input == "exit" {
            println!("👋 Goodbye!");
            break;
        }

        let correct_answer = Num2Words::new(number)
            .to_words()
            .unwrap_or_default()
            .to_lowercase();

        *answered.lock().unwrap() += 1;

        if user_input == correct_answer {
            println!("✅ Correct!");
            *correct.lock().unwrap() += 1;
            numbers.pop(); // فقط در صورت جواب صحیح حذف کن
        } else {
            println!("❌ Wrong. Correct: {}", correct_answer);
            *wrong.lock().unwrap() += 1;
        }
    }

    println!("\n🎉 You've practiced all numbers in the range!");
    print_summary(*correct.lock().unwrap(), *wrong.lock().unwrap(), total, *answered.lock().unwrap());
}



fn print_summary(correct: u32, wrong: u32, total: usize, answered: u32) {
    println!("\n📊 Summary:");
    println!("🧠 Total answered:  {}/{}", answered, total);
    println!("✅ Correct answers: {}", correct);
    println!("❌ Wrong answers:   {}", wrong);
}