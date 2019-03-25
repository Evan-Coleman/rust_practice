use std::fmt::Display;

fn main() {
    let x = String::from("Hello this is a longer string!");
    let y = "This is short!";
    let ann = "CONGRATS!";

    println!("{}", longest_with_an_announcement(&x, &y, ann));
}

// This example has a lifetime and generic in the function definition
// The return type is a reference with a lifetime and a bound trait of Display
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}