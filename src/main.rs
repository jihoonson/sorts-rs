extern crate sorts;
extern crate rand;
extern crate time;

use sorts::sort::SortAlgorithm;
use sorts::radix_sort::RadixSort;

fn main() {
    // integer sort
    // let len = 100_000_000;
    // let num_range: u32 = 1_000_000;
    // println!("Generating random numbers");
    // let numbers: Vec<u32> = (0..len).map(|_| rand::random::<u32>() % num_range).collect();
    //
    // //let sort: RadixSort = Sort::new();
    // let mut clone1 = numbers.clone();
    // println!("Running RadixSort::sort()");
    // let start = time::now();
    // //sort.sort(&mut clone1);
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
    println!("Generating random numbers");
    let len = 100_000_000;
    let numbers: Vec<f32> = (0..len).map(|_| rand::random::<f32>()).collect();

    //let sort: RadixSort = Sort::new();
    let mut clone1 = numbers.clone();
    println!("Running RadixSort::sort()");
    let start = time::now();
    //sort.sort(&mut clone1);
    RadixSort(&mut clone1).sort();
    let end = time::now();
    println!("Done! {} ms", (end-start).num_milliseconds());

    let mut clone2 = numbers.clone();
    println!("Running slice::sort()");
    let start = time::now();
    clone2.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let end = time::now();
    println!("Done! {} ms", (end-start).num_milliseconds());
}
