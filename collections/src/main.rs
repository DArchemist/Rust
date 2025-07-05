use std::collections::HashMap;

fn main() {
    // Vectors

    let v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3];

    let mut v2 = Vec::new();

    // Adding elements to vector
    v2.push(5);
    v2.push(3);

    // Getting elements from vector

    let second = &v2[1];

    let second = v.get(1);
    match second {
        Some(second) => println!("The second element is {second}"),
        None => println!("There is no third element."),
    }

    // Iterating over the values of a vector

    let v3 = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v4 = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // To hold different types inside the same vector

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Strings

    let mut s = String::new(); // Creating an empty string

    let data = "initial contents";

    let s = data.to_string();

    let s1 = String::from("initial contents");

    // Appending to a String with push_str and push

    let mut s2 = String::from("foo");
    s2.push_str("bar");

    let mut s3 = String::from("lo");
    s3.push('l');
    
    // Indexing: Rust strings don't support indexing! only slices

    // Iterating over strings

    for c in "Hello".chars() {
        println!("{c}");
    }

    for c in "Hello".bytes() {
        println!("{c}");
    }

    // Hash maps

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }



}
