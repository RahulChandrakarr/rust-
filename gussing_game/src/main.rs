// ============================================================================
// RUST GUESSING GAME - Learning Project
// ============================================================================
// This program demonstrates key Rust concepts:
// - Input/Output (std::io)
// - Random number generation (rand crate)
// - Variable declarations (let, mut)
// - Loops (for loop with range)
// - Conditionals (if/else if/else)
// - String manipulation and type conversion
// - Error handling with expect()
// ============================================================================

// IMPORTS - Bringing external functionality into our program
use std::io;        // Standard library for input/output operations
use rand::Rng;      // External crate for random number generation

fn main() {
    // PRINTING TO CONSOLE
    // println! is a macro (note the !) that prints text to the terminal
    println!("guessing game");
    println!("You have 3 chances to guess the number between 1 to 5");

    // VARIABLE DECLARATION - IMMUTABLE
    // 'let' creates an immutable variable (cannot be changed after creation)
    // rand::rng() creates a random number generator (updated API)
    // random_range(1..=5) generates a number from 1 to 5 (inclusive)
    let secret_number = rand::rng().random_range(1..=5);
    // println!("the secret number is: {}", secret_number); // Debug line - uncomment to cheat!

    // VARIABLE DECLARATION - MUTABLE
    // 'let mut' creates a mutable variable (can be changed)
    // bool type can be true or false
    let mut won = false;

    // FOR LOOP - Iterating over a range
    // 1..=3 creates an inclusive range (1, 2, 3)
    // 'attempt' is the loop variable that takes each value in the range
    for attempt in 1..=3 {
        // STRING FORMATTING
        // {} is a placeholder for variables in println!
        // \n creates a new line
        println!("\nAttempt {}/3:", attempt);
        println!("Please input your guessing number between 1 to 5:");
        
        // MUTABLE STRING CREATION
        // String::new() creates an empty, growable string
        // Must be mutable because read_line will modify it
        let mut guess: String = String::new();

        // INPUT/OUTPUT OPERATIONS
        // io::stdin() gets a handle to standard input (keyboard)
        // read_line() reads a line from input and stores it in the string
        // &mut guess passes a mutable reference to the string
        // expect() handles potential errors - if reading fails, program panics with this message
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        // TYPE CONVERSION AND SHADOWING
        // This creates a new variable with the same name (shadowing the old one)
        // guess.trim() removes whitespace and newlines
        // .parse() converts string to number
        // u32 is an unsigned 32-bit integer (0 to 4,294,967,295)
        // Better error handling with match instead of expect()
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue; // Skip to next iteration, don't count as an attempt
            }
        };

        // CONDITIONAL STATEMENTS - if/else if/else
        // == checks for equality
        // < and > are comparison operators
        if guess == secret_number {
            println!("🎉 You win!");
            won = true;     // Update the mutable variable
            break;          // Exit the loop early
        } else if guess < secret_number {
            println!("Too small!");
        } else {
            println!("Too big!");
        }
        
        // NESTED CONDITIONAL
        // Only show remaining attempts if not the last attempt
        if attempt < 3 {
            println!("You have {} attempts remaining.", 3 - attempt);
        }
    }

    // FINAL CONDITIONAL - Check game result
    // ! is the NOT operator (if NOT won)
    if won {
        println!("Congratulations! Let's play again sometime!");
    } else {
        println!("Game over! The number was {}. Better luck next time!", secret_number);
    }
}