#![feature(libc)]
extern crate rand;

use sort::SortAlgorithm;
use radix_sort::RadixSort;
use rand::Rng;

pub mod sort;

pub mod radix_sort;

// #[test]
fn test_u32() {
    let len = 100_000;
    let num_range: u32 = 1_000_000;
    let numbers: Vec<u32> = (0..len).map(|_| rand::random::<u32>() % num_range).collect();

    let mut clone1 = numbers.clone();
    RadixSort(&mut clone1).sort();

    let mut clone2 = numbers.clone();
    clone2.sort();

    assert_eq!(clone1, clone2);
}

// #[test]
fn test_f32() {
    let len = 100_000;
    let numbers: Vec<f32> = (0..len).map(|_| rand::random::<f32>()).collect();

    let mut clone1 = numbers.clone();
    RadixSort(&mut clone1).sort();

    let mut clone2 = numbers.clone();
    clone2.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    assert_eq!(clone1, clone2);
}

fn test_negative_f32() {
    let len = 100_000;
    let numbers: Vec<f32> = (0..len).map(|_| rand::random::<f32>() * -1.0f32).collect();

    let mut clone1 = numbers.clone();
    RadixSort(&mut clone1).sort();

    let mut clone2 = numbers.clone();
    clone2.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    assert_eq!(clone1, clone2);
}

#[test]
fn test_str() {
    let vec_len = 10;
    let max_str_len = 5;

    let strings: Vec<String> = (0..vec_len).map(|_|
        rand::thread_rng()
            .gen_ascii_chars()
            .take((rand::random::<u32>() % max_str_len + 1) as usize)
            .collect::<String>()
    ).collect();

    let mut clone1 = strings.clone();
    RadixSort(&mut clone1).sort();

    let mut clone2 = strings.clone();
    clone2.sort();

    assert_eq!(clone1, clone2);
}
