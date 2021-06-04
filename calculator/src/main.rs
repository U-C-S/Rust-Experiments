use std::io;
mod calculate;

fn main() {
    println!("The Calculator\nEnter the equation with spaces");

    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Something is wrong");

    let input_split: Vec<&str> = inp.split(" ").collect();
    if input_split.len() == 3 {
        let ans = calculate::calculate(&input_split);
        match ans {
            Some(ans) => print!("Answer: {}", ans),
            None => println!("Something is terribly wrong"),
        };
    } else {
        println!("Wrong! Enter like this: 2 + 3");
    }
}
