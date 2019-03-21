# 1.2
* in "println!" the bang stands for a macro, if it was ust "println" it would be a function call.

# 1.3
* cargo build
* cargo run (will rebuild automatically if changes happen)
* cargo check (makes sure code compiles, but doesn't produce exe -- FASTER THAN BUILD)
* cargo build --release (slower compile, faster binary)

# 2.0
* variables in Rust are immutable by default
    * use "mut" keyword to allow mutability
* Cargo.lock prevents upgrading dependencies without you telling it to
* cargo doc --open
    * builds documentation for all dependencies and opens in your browser
* Invalid input handling
    * .expect will crash the program
    * changing it to a match expression can handle it without crashing
        * { Ok(num) =>, Err(_) => continue }
            * Ok // Err come from the Result enum type

# 3.0
* To use a keyword "fn" as an identifier you must use a "Raw identifier" r#
    * let r#fn = ...
        * In this case, we are naming a variable fn, even though it is a reserved keyword

# 3.1
* const is always immuatable, and type must be defined up-front
    * Can't be the result of a fn or anything like that
    * Naming convention: ALL_CAPS_UNDERSCORES
    * Same name convention can apply to numeric literals
        * const MAX_POINTS: u32 = 100_000;