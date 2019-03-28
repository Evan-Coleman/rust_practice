use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    let simulated_user_specified_value = 20;
    let simulated_random_number = 3;

    generate_workout (
        simulated_user_specified_value,
        simulated_random_number
    );
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {        
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(35));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for  {} minutes!", expensive_result.value(intensity));
        }
    }
}

struct Cacher<T, X, Y> where T: Fn(X) -> Y, X: Hash + Eq + Copy, Y: Copy {
    calculation: T,
    value: HashMap<X, Y>,
}

impl<T, X, Y> Cacher<T, X, Y> where T: Fn(X) -> Y, X: Hash + Eq + Copy, Y: Copy {
    fn new(calculation: T) -> Cacher<T, X, Y> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    // If the calculation has already been called it will return a value, if not it will do the calculation
    fn value(&mut self, arg: X) -> Y {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            },
        }
    }
}