fn main() {
    let mut hello1 = String::from("Hello");
    let mut hello2 = "Hello".to_string();

    println!("{}, {}", hello1, hello2);

    hello1.push('!');
    hello2.push_str(" World!");

    println!("{}, {}", hello1, hello2);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let concat1 = String::from("Hello, ");
    let concat2 = String::from("world!");
    let concat3 = concat1 + &concat2;
    println!("{}", concat3);
    // This line won't work because concat1 is borrwoed when we assigned concat3
    // println!("{}", concat1);
    // This line will, work because we just used a ref to concat2
    println!("{}", concat2);

    let tic1 = "tic";
    let tic2 = "tac";
    let tic3 = "toe";

    let tictactoe = format!("{}-{}-{}", tic1, tic2, tic3);
    println!("{}", tictactoe);

    for c in "नमस्ते".chars() {
    println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
    println!("{}", b);
}
}
