use rand::prelude::*;
use std::time::Instant;
use text_io::read;

//      This Program is written by "U Chanakya Srinivas"
//          USN: 4ni18is106
//          Class: 4th SEM , Batch 1
//          Language: Rust (Similar to Modern C++)

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
        println!("1. Bubble \n2. Selection \n3. Merge \n4. Quick \n5. Exit()");

        let opt: Option<i8> = Some(read!());
        match opt {
            Some(1) => {
                let mut arr1 = arr.clone();

                let now = Instant::now();
                bubble_sort(&mut arr1);
                let elapsed = now.elapsed();

                //println!("The output of Bubble Sort is: {:?}", &arr);
                println!("Elapsed: {:.2?}", elapsed);
            }
            Some(2) => {
                let mut arr1 = arr.clone();

                let now = Instant::now();
                selection_sort(&mut arr1);
                let elapsed = now.elapsed();

                //println!("The output of Selection Sort is: {:?}", &arr1);
                println!("Elapsed: {:.2?}", elapsed);
            }
            Some(3) => {
                let mut arr1 = arr.clone();

                let now = Instant::now();
                merge_sort(&mut arr1, 0, size - 1);
                let elapsed = now.elapsed();

                //println!("The output of Merge Sort is: {:?}", &arr1);
                println!("Elapsed: {:.2?}", elapsed);
            }
            Some(4) => {
                let mut arr1 = arr.clone();

                let now = Instant::now();
                quick_sort(&mut arr1, 0, (size - 1) as isize);
                let elapsed = now.elapsed();

                //println!("The output of Quick Sort is: {:?}", &arr1);
                println!("Elapsed: {:.2?}", elapsed);
            }
            Some(5) => break,
            Some(_) => continue,
            None => continue,
        }
    }
}

//skipped the delays because the many inputs are given

fn bubble_sort(arr: &mut Vec<usize>) {
    let size = arr.len();

    for i in 0..size - 1 {
        for j in 0..size - i - 1 {
            //sleep(Duration::from_millis(100));
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn selection_sort(arr: &mut Vec<usize>) {
    let size = arr.len();

    for i in 0..size - 1 {
        let mut min = i;
        for j in i + 1..size {
            //sleep(Duration::from_millis(100));
            if arr[min] > arr[j] {
                min = j;
            }
        }
        arr.swap(i, min);
    }
}

fn merge_sort(arr: &mut Vec<usize>, low: usize, high: usize) {
    if low < high {
        let mid = low + (high - low) / 2;
        merge_sort(arr, low, mid);
        merge_sort(arr, mid + 1, high);
        _merge(arr, low, mid, high);
    }
}

fn quick_sort(arr: &mut Vec<usize>, low: isize, high: isize) {
    if low < high {
        let p = _partition(arr, low, high);
        quick_sort(arr, p + 1, high);
        quick_sort(arr, low, p - 1);
    }
}

//helpers all go here--------------
fn _merge(arr: &mut Vec<usize>, low: usize, mid: usize, high: usize) {
    let mut left_half = Vec::new();
    let mut right_half = Vec::new();

    for v in arr.iter().take(mid + 1).skip(low) {
        left_half.push(*v);
    }
    for v in arr.iter().take(high + 1).skip(mid + 1) {
        right_half.push(*v);
    }

    let lsize = left_half.len();
    let rsize = right_half.len();

    let mut l = 0;
    let mut r = 0;
    let mut a = low;

    while l < lsize && r < rsize {
        if left_half[l] < right_half[r] {
            arr[a] = left_half[l];
            l += 1;
        } else {
            arr[a] = right_half[r];
            r += 1;
        }
        a += 1;
    }

    while l < lsize {
        arr[a] = left_half[l];
        l += 1;
        a += 1;
    }

    while r < rsize {
        arr[a] = right_half[r];
        r += 1;
        a += 1;
    }
}

fn _partition(arr: &mut Vec<usize>, low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut i = low - 1;
    let mut j = high;

    loop {
        i += 1;
        while arr[i as usize] < arr[pivot] {
            i += 1;
        }
        j -= 1;
        while j >= 0 && arr[j as usize] > arr[pivot] {
            j -= 1;
        }
        if i >= j {
            break;
        } else {
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap(i as usize, pivot as usize);
    return i;
}

// Inputs : Bubble,Select, Merge, Quick -- Time in microseconds
// 1000   : 980   , 670  , 410  , 60
// 2000   : 3700  , 2560 , 830  , 125
// 3000   : 8650  , 5670 , 1340 , 195
// 4000   : 16250 , 9950 , 1710 , 260
// 5000   : 27200 , 15500, 2520 , 331
// 6000   : 41800 , 22280, 2980 , 390
// 10000  : 137500, 61100, 4950 , 660
