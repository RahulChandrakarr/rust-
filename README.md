# 🦀 Rust Learning Journey

Welcome to my Rust learning repository! This project documents my journey learning Rust programming through hands-on projects.

## 📚 What I've Learned

### 1. **Input/Output (std::io)**
- How to read user input from the terminal
- Using `io::stdin().read_line()` to capture user input
- Error handling with `.expect()` method
- Printing output with `println!` macro

```rust
use std::io;

let mut input = String::new();
io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
```

### 2. **Random Number Generation (rand crate)**
- Adding external dependencies in `Cargo.toml`
- Using `rand::thread_rng()` for random number generation
- Generating numbers in a specific range with `gen_range()`
- Understanding inclusive (`1..=5`) vs exclusive (`1..5`) ranges

```rust
use rand::Rng;

let secret_number = rand::thread_rng().gen_range(1..=5);
```

### 3. **Variable Declarations**
- **Immutable variables**: `let variable_name = value;`
- **Mutable variables**: `let mut variable_name = value;`
- **Type annotations**: `let number: u32 = 42;`
- **Shadowing**: Creating new variables with the same name

```rust
let x = 5;           // Immutable
let mut y = 10;      // Mutable
let z: u32 = 15;     // With type annotation
let x = x + 1;       // Shadowing (new variable)
```

### 4. **Data Types**
- **Integers**: `u32` (unsigned 32-bit), `i32` (signed 32-bit)
- **Strings**: `String` (growable) vs `&str` (string slice)
- **Booleans**: `bool` (true/false)
- **Type conversion**: `.parse()` method for string to number conversion

### 5. **Loops in Rust**
I learned about all 7 types of loops in Rust:

#### **Basic Loop Types:**
1. **`loop`** - Infinite loop
```rust
loop {
    // Runs forever until 'break'
    break;
}
```

2. **`while`** - Conditional loop
```rust
while condition {
    // Runs while condition is true
}
```

3. **`for`** - Iterator loop
```rust
for i in 1..=5 {
    // Runs for each value in range
}
```

4. **`while let`** - Pattern matching loop
```rust
while let Some(value) = stack.pop() {
    // Runs while pattern matches
}
```

#### **Advanced Loop Features:**
5. **Loop control**: `break` and `continue`
6. **Labeled loops**: Breaking out of nested loops
7. **Returning values from loops**: `break value;`

### 6. **Conditional Statements**
- `if` / `else if` / `else` statements
- Comparison operators: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Logical operators: `&&` (AND), `||` (OR), `!` (NOT)

```rust
if guess == secret_number {
    println!("You win!");
} else if guess < secret_number {
    println!("Too small!");
} else {
    println!("Too big!");
}
```

### 7. **Error Handling**
- Using `.expect()` for basic error handling
- Understanding that Rust functions can return `Result<T, E>`
- Using `match` statements for better error handling
- Graceful error handling prevents crashes
- `continue` to skip iterations on invalid input

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Please enter a valid number!");
        continue; // Skip to next iteration
    }
};
```

### 8. **String Manipulation**
- Creating new strings: `String::new()`
- Trimming whitespace: `.trim()`
- String formatting with `println!("{}", variable)`

## 🎮 Projects

### Guessing Game (`gussing_game/`)
A number guessing game that demonstrates:
- User input and output
- Random number generation
- Loops with limited attempts
- Conditional logic
- Type conversion
- Error handling

**Features:**
- 3 attempts to guess a number between 1-5
- Feedback on each guess (too high/too low)
- Win/lose conditions
- Attempt counter

**How to run:**
```bash
cd gussing_game
cargo run
```

## 🛠️ Setup and Dependencies

### Prerequisites
- Rust installed (https://rustup.rs/)
- Cargo (comes with Rust)

### Dependencies Used
- `rand = "0.9.2"` - For random number generation

### Project Structure
```
rust/
├── README.md                 # This file
├── main.rs                   # Simple hello world
├── gussing_game/            # Main learning project
│   ├── Cargo.toml           # Project configuration
│   └── src/
│       └── main.rs          # Game implementation
└── rust-by-example/         # Rust learning resources
```

## 🚀 Running the Code

1. **Simple Hello World:**
```bash
rustc main.rs
./main
```

2. **Guessing Game (Recommended):**
```bash
cd gussing_game
cargo run
```

## 📖 Key Rust Concepts Learned

1. **Ownership and Borrowing** (Basic understanding)
   - References with `&` and `&mut`
   - Mutable vs immutable references

2. **Memory Safety**
   - No null pointer exceptions
   - Compile-time memory safety checks

3. **Pattern Matching** (Introduction)
   - Basic `match` statements
   - `if let` and `while let` patterns

4. **Cargo Package Manager**
   - Creating new projects with `cargo new`
   - Managing dependencies in `Cargo.toml`
   - Building and running with `cargo run`

## 🎯 Next Learning Goals

- [ ] Learn about structs and enums
- [ ] Understand ownership and borrowing in depth
- [ ] Explore pattern matching with `match`
- [ ] Learn about traits and generics
- [ ] Build a more complex project (maybe a CLI tool)
- [ ] Explore async programming in Rust

## 📝 Notes

- Rust's compiler is very helpful with error messages
- The `!` in `println!` indicates it's a macro, not a function
- Rust prevents many common programming errors at compile time
- The community and documentation are excellent resources

---

*This README will be updated as I continue learning Rust! 🦀*