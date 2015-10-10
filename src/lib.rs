#![feature(convert)]
#![feature(vec_push_all)]
extern crate rand;

use sort::Sort;
use radix_sort::RadixSort;

pub mod sort;

pub mod radix_sort;

#[test]
fn test_sort() {
    let len = 100_000;
    let num_range: u32 = 1_000_000;
    let numbers: Vec<u32> = (0..len).map(|_| rand::random::<u32>() % num_range).collect();
    
    let sort: RadixSort = Sort::new();
    let mut clone1 = numbers.clone();
    sort.sort(&mut clone1);

    let mut clone2 = numbers.clone();
    clone2.sort();

    assert_eq!(clone1, clone2);
}
