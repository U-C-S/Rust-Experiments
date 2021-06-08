use rand::prelude::*;
use std::thread::sleep;
use std::time::{Duration, Instant};
use text_io::read;

fn main() {
    println!("Enter the Array Size: ");
    let size: usize = read!();

    println!("Randomly Generated Elements are: ");
    let mut arr: Vec<usize> = Vec::new();
    for _i in 0..size {
        let num = rand::thread_rng().gen_range(0..1000);
        print!("{} ", num);
        arr.push(num);
    }

    loop {
        println!("\nChoose a sort option: ");
        println!("1. Bubble \n2. Selection \n3. Merge \n4. Exit()");
        let opt: Option<i8> = Some(read!());

        match opt {
            Some(1) => {
                println!("\nThe output of Bubble Sort is: ");
                let arr1 = arr.clone();
                let now = Instant::now();
                let arr1 = bubble_sort(size, arr1);
                let elapsed = now.elapsed();
                print!("{:?}", &arr1);
                println!("Elapsed: {:.2?}", elapsed);
            }
            Some(2) => {
                println!("\nThe output of Selection Sort is: ");
                let arr1 = arr.clone();
                let now = Instant::now();
                let arr1 = selection_sort(size, arr1);
                let elapsed = now.elapsed();
                print!("{:?}", &arr1);
                println!("Elapsed: {:.2?}", elapsed);
            }
            Some(3) => {}
            Some(4) => break,
            Some(_) => continue,
            None => continue,
        }
    }
}

fn bubble_sort(size: usize, mut arr: Vec<usize>) -> Vec<usize> {
    for i in 0..size - 1 {
        for j in 0..size - i - 1 {
            sleep(Duration::from_secs(1));
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }

    return arr;
}

fn selection_sort(size: usize, mut arr: Vec<usize>) -> Vec<usize> {
    for i in 0..size {
        let mut min = i;
        for j in 0..size {
            sleep(Duration::from_secs(1));
            if arr[min] > arr[j] {
                min = j;
            }
        }
        arr.swap(i, min);
    }

    return arr;
}

//fn merge_sort() {}

//fn quick_sort() {}
