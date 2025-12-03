// This file is for learning how variables work in Rust.
// We cover:
// - Immutable vs mutable variables
// - Type annotations (i32, u32)
// - Shadowing (re‑declaring a variable name)

fn main() {
    // ------------------------------
    // 1. Immutable variable (default)
    // ------------------------------
    // `let` creates an immutable variable by default (cannot be changed).
    // `i32` means a signed 32‑bit integer: it can store negative and positive numbers.
    let x: i32 = 5;
    println!("x is {}", x);

    // ------------------------------
    // 2. Shadowing (re‑declaring `x`)
    // ------------------------------
    // Here we create a NEW `x` that shadows (hides) the old one.
    // Type and name are the same, but value is different.
    let x: i32 = 6;
    println!("shadowed x is {}", x);

    // ------------------------------
    // 3. Mutable variable with type inference
    // ------------------------------
    // `mut` makes a variable mutable so we can change its value.
    // No type is written here; Rust infers `i32` from the value `10`.
    let mut y = 10;
    println!("y is {}", y);
    y = 20; // OK because `y` is mutable
    println!("updated y is {}", y);

    // ------------------------------
    // 4. Using `u32` instead of `i32`
    // ------------------------------
    // `u32` = *unsigned* 32‑bit integer: only 0 and positive numbers.
    // We use `u32` for values that should never be negative:
    // counts, sizes, ages, indexes, etc.
    let age: u32 = 25;
    println!("age (u32) is {}", age);

    // In the guessing game we used `u32` for the guessed number and secret number
    // because a "guess" or "random number between 1 and 5" cannot be negative.
}
