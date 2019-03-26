fn main() {
    // copy_practice();

    let s1 = String::from("Some String");

    take_ownership(s1);

// This line won't run because s is now out of scope because it was borrowwed by take_ownership
    // println!("{}", s);


    let s2 = String::from("Some String");

    let s3 = take_ownership_and_give_back(s2);

    println!("s2: {}", s3);
}

fn take_ownership_and_give_back(st: String) -> String {
    println!("{}", st);

    st
}

fn take_ownership(st: String) {
    println!("{}", st);
}

fn copy_practice() {
    let mut x = 5;
    
    let y = x;

// Since primitive types like this are afixed and known size, they are stack allocated
// When we set y = x, we are creating a new entry on the stack for y of the value x
// When we change x after the fact, y stays the same
    x = 2;
    println!("X: {}, Y: {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1;

// The following line is invalid, because s2 = s1 invalidates s1 and now s2
// holds the stack allocated portion of s1
    // println!("{}, world!", s1);

    println!("{}, world!", s2);

    let s3 = String::from("HELLO");
    // .clone() creates a deep copy, so s3 is not invalidated
    let s4 = s3.clone();

    println!("s3: {}, s4: {}", s3, s4);
}