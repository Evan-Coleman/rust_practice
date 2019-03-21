fn main() {
    // cond_stuff();

    let num = 50;
    println!("Fib of {} is: {}", num, fib(num));
}

fn fib(mut num: usize) -> usize {
    let mut a = 0;
    let mut b = 1;

    for _ in 1..num {
        let tmp = a;
        a = b;
        b = tmp + a;
    }

    b
}

fn cond_stuff() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}",number);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }
    for number in (1..4).rev() {
        println!("{}", number);
    }
}