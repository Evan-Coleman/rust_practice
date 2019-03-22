#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle1 is: {} square pixels",
        area(&rect1)
    );

// main retains ownership of rect1 with the ref types
    println!("Rectangle height: {}", rect1.height);

    let rect2 = Rectangle { width: 20, height: 20 };

    println!(
        "The area of the rectangle2 is: {} square pixels",
        rect1.area()
    );

    println!("Rect 1 can hold Rect2: {}", rect1.can_hold(&rect2));
    println!("Rect 2 can hold Rect1: {}", rect2.can_hold(&rect1));

    let sq = Rectangle::square(5);

    println!("Square height and width: {}, {}", sq.height, sq.width);
}

// When the type is a reference &Rectangle, the ownership is not transferred
fn area(rectangle: &Rectangle) -> u32 {
    println!("rect1 is {:#?}", rectangle);

    rectangle.width * rectangle.height
}