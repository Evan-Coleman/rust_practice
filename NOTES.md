# 1.2 - Hello World
* in "println!" the bang stands for a macro, if it was ust "println" it would be a function call.

# 1.3 - Cargo
* cargo build
* cargo run (will rebuild automatically if changes happen)
* cargo check (makes sure code compiles, but doesn't produce exe -- FASTER THAN BUILD)
* cargo build --release (slower compile, faster binary)

# 2.0 - Guessing Game
* variables in Rust are immutable by default
    * use "mut" keyword to allow mutability
* Cargo.lock prevents upgrading dependencies without you telling it to
* cargo doc --open
    * builds documentation for all dependencies and opens in your browser
* Invalid input handling
    * .expect will panic
    * changing it to a match expression can handle it without a panic
        * { Ok(num) =>, Err(_) => continue }
            * Ok // Err come from the Result enum type

# 3.0 - Keywords
* To use a keyword "fn" as an identifier you must use a "Raw identifier" r#
    * let r#fn = ...
        * In this case, we are naming a variable fn, even though it is a reserved keyword

# 3.1 - Variables
* const is always immuatable, and type must be defined up-front
    * Can't be the result of a fn or anything like that
    * Naming convention: ALL_CAPS_UNDERSCORES
    * Same name convention can apply to numeric literals
        * const MAX_POINTS: u32 = 100_000;

# 3.2 - Data Types
* Arrays are fixed length && stack allocated instead of heap
    * let a: [i32; 5] = [1, 2, 3, 4, 5];
        * Length of "a" is 5, it cannot shrink or grow
        * Type of "a" is i32, all data must match this

# 3.3 - Functions
* Function names: snake_case (all lower)
* For a statement, there is no need for an ending semicolon
    * fn plus_one(x: i32) -> i32 { x + 1 }

# 3.5 - Control Flow
* Loop can be stopped with break, or can return a value if used in a let statement
    * break counter * 2;