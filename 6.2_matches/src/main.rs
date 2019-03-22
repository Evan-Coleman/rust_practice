fn main() {
    // matches();
    // fizz_buzz(100);

    let five = Some(5);
    let six = println!("{:?}", plus_one(five));
    let none = println!("{:?}", plus_one(None));
}

fn matches() {
    for i in 1..10 {
        match i {
            4 => println!("HEY"),
            9 => println!("YOU"),
            _ => println!("WHATEVER"),
        }
    }
}

// Just for fun!
fn fizz_buzz(num: i32) {
    for i in 1..=num {
        let mut s = String::from("");
        if i % 3 == 0   {s.push_str("Fizz");}
        if i % 5 == 0   {s.push_str("Buzz");}
        if s.is_empty() {s.push_str(&i.to_string())}
        println!("{}", s);
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}