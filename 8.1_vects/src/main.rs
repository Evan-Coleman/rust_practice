fn main() {
    let mut v1: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];

    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    // Causes a panic
    // println!("{}", v1[4]);

    // No panic here, just returns a None
    println!("{:?}", v1.get(4));

    // Valid index, so 8 prints!
    println!("{}", &v1[3]);

    // i is just a reference to the memory location, so a deref is needed
    for i in &mut v2 {
        *i += 50;
    }

    for i in &v2 {
        println!("{}", i);
    }

    let row = vec![
        SpreadSheetRow::Num(3),
        SpreadSheetRow::Percent(0.99),
        SpreadSheetRow::Text(String::from("Heya!")),
    ];

    for i in &row {
        println!("{:?}", i);
    }

}

#[derive(Debug)]
enum SpreadSheetRow {
    Num(i32),
    Percent(f64),
    Text(String),
}