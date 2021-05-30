use std::{
    fs::{self, File},
    io::{self, ErrorKind},
};

//pub mod units;

fn main() {
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

    let mut x;
    x = vec!["da", "bl", ";dad"];
    x.push("value");
    x.get(1);

    let meow = "Meow meow".to_string();
    let meow = "bow";

    meow.bytes();
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s1 = s1 + &s2;
    print!("{}", s1);

    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    const ADD: i32 = 2;

    let mut scores: HashMap<String, i32> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
}
