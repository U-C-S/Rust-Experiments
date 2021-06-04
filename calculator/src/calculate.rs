pub fn calculate(input: &Vec<&str>) -> Option<f32> {
    let x1: f32 = input.get(0).expect("error").trim().parse().unwrap();
    let x2: f32 = input.get(2).expect("error").trim().parse().unwrap();

    match input.get(1) {
        Some(&"+") => {
            let ans = &x1 + &x2;
            Some(ans)
        }
        Some(&"-") => {
            let ans = &x1 - &x2;
            Some(ans)
        }
        Some(&"*") => {
            let ans = &x1 * &x2;
            Some(ans)
        }
        Some(&"/") => {
            let ans = &x1 / &x2;
            Some(ans)
        }
        Some(_) => None,
        None => None,
    }
}
