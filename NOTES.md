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

# 4.1 - Ownership
* For an item to be put on the stack, it must have a known and fixed size
* Complex types like String live on both the stack and the heap
    * Stack:
        * ptr: pointer to the heap where the actual data is stored
        * len: length of the string
        * capacity: how much it can hold
    * Heap:
        * contiguous section of memory holding each char
            * [0] = 'h', [1] = 'e', [2] = 'l', [3] = 'l', [4] = 'o'
* A copy of these complex types isn't like a primitive type copy
    * When you copy a string, you're making a copy of the stack allocated portion
        * Because of this, both variables will point to the same space on the heap
    * This is like a *shallow copy* in other languages, but Rust invalidates the first variable so this is just a move and not a copy

# 6.2 - Match
* The Option<T> type is used for something that could be null
    * Rust doesn't use null for safety, if something CAN be null use Option
        * This is because it forces you to handle both cases of null && !null
            * Some == !null
            * None == null

# 7.2 - Modules / Scope
* When exporting a module to a new file, the filename is the containing module and is directly accessable from "mod name;" up by use statements

# 8.1 - Vectors
* To access data from a vector you can use 2 methods
    * v[x] - will cause a panic if you try to access outside the bounds
    * v.get(x) - Will return some(&element) or None, no panic
* Debug print with "{:?}"
    * If it's a custom type you are debug printing, add the debu attribute above it
        * #[derive(Debug)]

# 8.2 - Strings
* str vs String
    * String is a growable, heap-allocated data structure
        * Useful when you want to own the string and mutate it
    * str immutable fixed-length string in memory (heap, stack, or binary)
        * Good for function params for read-only view of a string
* Indexing strings "s[0]" isn't safe because some UTF-8 strings are more than 1 byte
    * slicing can work, if you know the size of the chars you are workign with

# 8.3 - Hash Maps
* By default HashMaps own both keys and values passed into them
    * You can insert references, but they must be guaranteed to be alive as long as the hashmap is

# 9.1 - Panic!
* Panic will crash the program out and unwind the stack before finishing
    * You can specifiy an abort only panic in your Cargo.toml file
        * Add [profile.release] panic = 'abort'

# 9.3 - When to Panic vs Result
* If the error is unrecoverable, or you don't know how to deal with it
    * panic!
* If There's a retry-able error
    * result
* unwrap / expeect are good for prototyping before you've decided how to handle errors

# 10.2 - Traits
* Traits are similar to interfaces in other lanuguages -- Contracts for custom types
* A default implementation can be defined in the trait
    * To use the default, you must specify an empty impl block of that trait on the type
* Trait bounds can bound a function to only take in / return something that implements a certain trait
    * pub fn notify<T: Summary>(item: T) {}
    Trait bounds can be chained together with the "+" symbol

# 10.3 - Lifetimes
* Syntax: 'a = hyphen + single lower-case character
* 'static - a lifetime for things like string-literals. stored directly in the binary so is always available to the program

# 11.1 - Testing
* Tests need annotations
    * For the module: #[cfg(test)] mod tests{}
    * Each test needs: #[test]
    * Special annotation: #[should_panic]
        * Can add to this: #[should_panic(expected = "Panic message to expect")]
* Macros for testing
    * assert!(a_bool_value)
    * assert_eq!(a,b) <- a,b must be the same to pass
    * assert_ne!(a,b) <- a,b must be different to pass
        * These macros can contain custom messages
            * result = true; assert!(a_bool_value, "Custom message: {}", result);

# 11.2 - Test Running
* cargo test {name} : Runs a specific test
* cargo test -- --ignored : Runs all tests with the ignored annotation
    * Ignored annotation: #[ignore]
* By default, all tests run in parallel
    * --test-threads=1 : for tests that may have race conditions
* cargo test -- --nocapture : Shows output from passing tests too

# 11.3 - Test Organization
* Unit testing: In src directory in each file with the code you are testing, can choose to test private functions
    * You can test private functions by bringing the module into the scope of the test
* Integration testing: entirely external to your library. Tests your code just as any other external code would through the public API
    * Located in the tests folder
* When you want some common functionality (like a setup function) put it in a /common/ folder
    * Files in subdirectories of the tests dir do not get ran as tests

# 12.3 - CMD program
* adding a "?" to the end of a function call will return errors from that function if there are any

# 12.6 - Error printing
* cargo run to poem.txt > output.txt
    * Will create a file output.txt with the correct output of the file
* cargo run > output.txt
    * Will create a file output.txt error "not enough arguments"
        * This is not expected, we only want successful output in the file, not errors
* the macro eprintln!() is more appropriate for writing errors to screen
    * as opposed to println!()

# 13.1 - Closures
* Basic syntax of a closure
    * let expensive_closure = |param| { function; return_val }
* Build a cacher by using trait bound (Fn, FnMut, FnOnce) on the generic type to ensure a function is passed in
* To add multiple trait bounds, use syntax: "X: Hash + Eq + Copy"

# 13.2 - Iterators
* The Iterator trait has 2 parts
    * value: a type that you want to iterate over
    * next: Only method you need to impl, will return a "Some(self.value)", or "None" if done
* Iterators are lazy, meaning that nothing happens until you work on it. So, if you don't call .next() you won't get a value, the first time you do call it, you'll get the first value.
* Maps take closures that can act on the data they are mapping!
    * map is an iterator adaptor which is lazy, .collect() is needed to turn the result into a collection

# 14.1 - Release Profiles
* cargo has 2 main profiles: dev / release each with corresponding opt-levels in Cargo.toml
    * Dev profile: cargo build
        * opt-level = 0
    * Release profile: cargo build --release
        * opt-level = 3
* opt-level can be from 0 to 3, and 0 compiles fastest, but the binary will run slower, opposite for 3

# 14.2 - Publishing to crates.io
* Documentation comment: "///"
    * Generates HTML documentation
    * Supports Markdown
* "cargo doc" will run the rustdoc tool and puts the documentation in "target/doc" directory
    * Automatically open in your browser after making: "cargo doc -- open"
* Common sections
    * Examples
        * If your example has code in it, "cargo test" will test it!
            * For this reason it might be good to incldue asserts in the examples
    * Panics
    * Errors
    * Safety
* "//!" comments
    * Commonly added to crate root file "src/lib.rs", or inside modules to document the crate/module as a whole
* Reexporting items using "pub use"
    * This allows for a more convient public API, so other people using your code don't have to have very largely nested use statements
* To publish crates to crates.io, you need to get an API key from them
    * Once you have the API key: cargo login {API KEY}
        * Stores credentials to ~/.cargo/credentials
            * Make sure this doesn't get added to your repo!
    * To publish you must add to the Cargo.toml file
        * [package]
            * name = "{UNIQUE NAME TO CRATES.IO}"
                * Must be unique to ALL crates on the site
            * license = "MIT"
                * List of license names: https://spdx.org/licenses/
                    * Multiple licenses with: "OR"
            * version = "0.1.0"
            * authors = ["Your Name <you@example.com>"]
            * edition = 2018
            * description = "Description of crate"
        * "cargo publish"
            * to update update version, then run again
                * Rules: https://semver.org/
        * Prevent others from adding a version of your crate to future projects
            * cargo yank --vers 1.0.1
                * cargo yank --vers 1.0.1 --undo

# 15.1 BOX
* Box<T> allows you to put data on the heap instead of stack
    * Pointer to heap remains on stack
    * Usage scenarios
        * Type where the size cant be known at compile time
        * Transfer large data without copying
        * To own a value not of a type, but which implements a specific trait
* Recursive types
    * cons list
        * "to cons x onto y"
            * construct new container instance by putting the element x at the start of this new container, followed by the container y
        * Last item in the list contains a value called Nil
            * This is different from "null" and "nil"
                * null == invalid
                * nil == absent value
        * Generally not used, a Vec<T> usually better
        * enum List { Cons(i32, List), Nil } -- BAD
            * Compiler cant determine size, so wont compile
        * enum List { Cons(i32, Box<List>), Nil } -- GOOD
            * The box is a fixed size since it is just a pointer, will compile