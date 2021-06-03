use std::io;

fn main() {
    println!("The Calculator\nEnter the equation with spaces");

    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Something is wrong");

    let input_split: Vec<&str> = inp.split(" ").collect();
    if input_split.len() == 3 {
        let x1: f32 = input_split.get(0).expect("error").trim().parse().unwrap();
        let x2: f32 = input_split.get(2).expect("error").trim().parse().unwrap();

        match input_split.get(1) {
            Some(&"+") => {
                let ans = &x1 + &x2;
                print!("Answer: {}", ans);
            }
            Some(&"-") => {
                let ans = &x1 - &x2;
                print!("Answer: {}", ans);
            }
            Some(&"*") => {
                let ans = &x1 * &x2;
                print!("Answer: {}", ans);
            }
            Some(&"/") => {
                let ans = &x1 / &x2;
                print!("Answer: {}", ans);
            }
            Some(_) => println!("This is not a correct operation"),
            None => println!("Illegal"),
        }
    } else {
        println!("Wrong! Enter like this: 2 + 3");
    }
}
