use std::collections::HashMap;

fn main() {
    let mut scores1 = HashMap::new();

    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Yellow"), 50);

    println!("{:?}", scores1);

    let teams = vec![String::from("Orange"), String::from("Green")];
    let initial_scores = vec![10, 50];

    // This line will  create a hashmap from teams and initial scores zipped together
    // since both the variables we are zipping already are typed we can set the hashmap
    // key and values to be _ types which will automatically infer
    let mut scores2: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores2);

    let team_name = String::from("Orange");
    let score1 = scores2.get(&team_name);
    println!("{:?}", score1);
    match score1 {
        Some(i) => println!("{}", i),
        None => println!("NONE"),
    }

    let score2 = scores2.get(&String::from("Blue"));
    match score2 {
        Some(i) => println!("{}", i),
        None => println!("NONE"),
    }

    for (key, value) in &scores2 {
        println!("{}: {}", key, value);
    }

    println!("---------");
    other_stuff();
}

fn other_stuff() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Orange"), 25);

    println!("{:?}", scores);

    // Overwrite
    scores.insert(String::from("Blue"), 10);
    // If entry does not exist, will insert 20
    scores.entry(String::from("Brown")).or_insert(20);

    println!("{:?}", scores);


    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
    // Grab the count from the text and if it doesn't exist yet add it with a value of 0
    let count = map.entry(word).or_insert(0);
    // Deref needed to modify value
    *count += 1;
    }

    println!("{:?}", map);
}
