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