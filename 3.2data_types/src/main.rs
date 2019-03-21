fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("The first value of the tuple is is: {}", tup.0);

    let (x, y, z) = tup; // Destructuring
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let a = [1, 2, 3, 4, 5];
    let index = 10;

    println!("Out of bounds index result: {}", a[index]);
}
