use rand::prelude::*;
use std::cmp::Ordering;
use std::thread::sleep;
use std::time::{Duration, Instant};
use text_io::read;

fn main() {
    println!("Enter the array size:");
    let size: i32 = read!();

    println!("Elements are: \n");
    let mut arr: Vec<i16> = Vec::new();
    for _i in 0..size {
        let num = rand::thread_rng().gen_range(0..200);
        print!("{} ", num);
        arr.push(num);
    }

    loop {
        println!("\nChoose a option: ");
        println!("1. Binary Search \n2. Linear Search \n3. Exit()");
        let opt: Option<i8> = Some(read!());

        match opt {
            Some(1) => {}
            Some(2) => {
                println!("Enter the element to be searched: ");
                let item: i16 = read!();
                let now = Instant::now();
                let pos = linear_search(item, arr.clone());
                let elapsed = now.elapsed();
                println!("Elapsed: {:.2?}", elapsed);
                println!("Element at index: {}", pos);
            }
            Some(3) => break,
            Some(_) => continue,
            None => continue,
        }
    }
}

fn linear_search(item: i16, arr: Vec<i16>) -> isize {
    let size = arr.len() as isize;

    for i in 0..size {
        sleep(Duration::from_millis(100));
        let curr = *arr.get(i as usize).unwrap();
        if item == curr {
            return i + 1;
        }
    }

    return 0;
}

pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;

        match item.cmp(&arr[mid]) {
            Ordering::Less => right = mid,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => left = mid + 1,
        }
    }
    None
}
