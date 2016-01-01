extern crate sorts;
extern crate rand;
extern crate time;

use sorts::sort::SortAlgorithm;
use sorts::radix_sort::RadixSort;
use rand::Rng;

fn main() {
    // integer sort
    // let len = 100_000_000;
    // let num_range: u32 = 1_000_000;
    // println!("Generating random numbers");
    // let numbers: Vec<u32> = (0..len).map(|_| rand::random::<u32>() % num_range).collect();
    //
    // let mut clone1 = numbers.clone();
    // println!("Running RadixSort::sort()");
    // let start = time::now();
    // RadixSort(&mut clone1).sort();
    // let end = time::now();
    // println!("Done! {} ms", (end-start).num_milliseconds());
    //
    // let mut clone2 = numbers.clone();
    // println!("Running slice::sort()");
    // let start = time::now();
    // clone2.sort();
    // let end = time::now();
    // println!("Done! {} ms", (end-start).num_milliseconds());

    // float sort
    // println!("Generating random numbers");
    // let len = 10_000_000;
    // let numbers: Vec<f32> = (0..len).map(|_| rand::random::<f32>()).collect();
    //
    // //let sort: RadixSort = Sort::new();
    // let mut clone1 = numbers.clone();
    // println!("Running RadixSort::sort()");
    // let start = time::now();
    // RadixSort(&mut clone1).sort();
    // let end = time::now();
    // println!("Done! {} ms", (end-start).num_milliseconds());
    //
    // let mut clone2 = numbers.clone();
    // println!("Running slice::sort()");
    // let start = time::now();
    // clone2.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    // let end = time::now();
    // println!("Done! {} ms", (end-start).num_milliseconds());

    // string sort
    println!("Generating random strings");
    let len = 10_000_00;
    let max_str_len = 4;
    let strings: Vec<String> = (0..len).map(|_|
        rand::thread_rng()
            .gen_ascii_chars()
            // .take((rand::random::<u32>() % max_str_len + 1) as usize)
            .take(max_str_len)
            .collect::<String>()
    ).collect();

    let mut clone1 = strings.clone();
    println!("Running RadixSort::sort()");
    let start = time::now();
    RadixSort(&mut clone1).sort();
    let end = time::now();
    println!("Done! {} ms", (end-start).num_milliseconds());

    let mut clone2 = strings.clone();
    println!("Running slice::sort()");
    let start = time::now();
    clone2.sort();
    let end = time::now();
    println!("Done! {} ms", (end-start).num_milliseconds());
}
