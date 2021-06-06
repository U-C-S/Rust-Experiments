use rand::prelude::*;
use std::cmp::Ordering;
use std::thread::sleep;
use std::time::{Duration, Instant};
use text_io::read;

fn main() {
    println!("Enter the array size:");
    let size: i32 = read!();

    println!("Elements are: \n");
    let mut arr: Vec<u16> = Vec::new();
    for _i in 0..size {
        let num = rand::thread_rng().gen_range(0..1000);
        print!("{} ", num);
        arr.push(num);
    }

    loop {
        println!("\nChoose a option: ");
        println!("1. Binary Search \n2. Linear Search \n3. Exit()");
        let opt: Option<i8> = Some(read!());

        match opt {
            Some(1) => {
                let arr_sorted = sort(arr.clone());
                print!("\nSorted list: ");
                for i in &arr_sorted {
                    print!("{} ", i);
                }
                println!("\nEnter the element to be searched: ");
                let item: u16 = read!();
                let arr1 = arr_sorted.clone();
                let now = Instant::now();
                let pos = binary_search(&item, arr1);
                let elapsed = now.elapsed();
                println!("Elapsed: {:.2?}", elapsed);
                println!("Element at index: {}", pos);
            }
            Some(2) => {
                println!("Enter the element to be searched: ");
                let item: u16 = read!();
                let arr1 = arr.clone();
                let now = Instant::now();
                let pos = linear_search(&item, arr1);
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

fn linear_search(item: &u16, arr: Vec<u16>) -> usize {
    let size = arr.len();

    for i in 0..size {
        sleep(Duration::from_millis(100));
        let curr = arr.get(i).unwrap();
        if item == curr {
            return i + 1;
        }
    }

    return 0;
}

pub fn binary_search(item: &u16, arr: Vec<u16>) -> usize {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        sleep(Duration::from_millis(100));
        let mid = left + (right - left) / 2;

        match item.cmp(&arr[mid]) {
            Ordering::Less => right = mid,
            Ordering::Equal => return mid + 1,
            Ordering::Greater => left = mid + 1,
        }
    }
    return 0;
}

fn sort(arr: Vec<u16>) -> Vec<u16> {
    let mut arr = arr;
    let size = arr.len();
    let mut i: usize = 0;
    while i < size - 1 {
        let mut j = i + 1;
        while j < size {
            if arr[i] > arr[j] {
                arr.swap(i, j);
            }

            j += 1;
        }

        i += 1;
    }

    return arr;
}
