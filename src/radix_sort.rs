extern crate libc;

use sort::SortAlgorithm;
use std::intrinsics::write_bytes;

extern {
    fn shift_left(f: *mut libc::c_void, pass: libc::c_ushort) -> libc::size_t;
    fn shift_right(f: *mut libc::c_void, pass: libc::c_ushort) -> libc::size_t;
}

pub struct RadixSort<'a, T:'a>(pub &'a mut T);

impl<'a> SortAlgorithm<'a> for RadixSort<'a, Vec<u32>> {

    fn sort(&'a mut self) {
        let &mut RadixSort (ref mut numbers) = self;

        let mut tmp: Vec<u32> = vec![0; numbers.len()];
        let mut positions: Vec<usize> = vec![0; 256];
        sort_u32(numbers,
            &mut tmp,
            &mut positions,
            0,
            false);
    }
}

impl<'a> SortAlgorithm<'a> for RadixSort<'a, Vec<f32>> {

    fn sort(&'a mut self) {
        let &mut RadixSort (ref mut numbers) = self;

        let mut tmp: Vec<f32> = vec![0.0f32; numbers.len()];
        let mut positions: Vec<usize> = vec![0; 256];
        sort_f32(numbers,
            &mut tmp,
            &mut positions,
            0,
            false);
    }
}

impl<'a> SortAlgorithm<'a> for RadixSort<'a, Vec<String>> {

    fn sort(&'a mut self) {
        let &mut RadixSort (ref mut strings) = self;

        let mut pointers: Vec<&String> = Vec::with_capacity(strings.len());
        let mut tmp: Vec<&String> = Vec::with_capacity(strings.len());
        let mut positions: Vec<usize> = vec![0; 256];

        let mut max_len: u16 = 0;
        for s in strings.iter() {
            if max_len < s.len() as u16 {
                max_len = s.len() as u16
            }
            pointers.push(&s);
        }

        sort_str(&mut pointers,
            &mut tmp,
            &mut positions,
            max_len,
            false);
    }
}

fn sort_u32(input: &mut Vec<u32>,
    output: &mut Vec<u32>,
    positions: &mut Vec<usize>,
    pass: u16,
    need_copy: bool) {

    // Count
    for n in input.iter() {
        let key: usize = ((n >> (pass * 8)) as u8) as usize;
        positions[key] += 1;
    }

    // Calculate positions
    for i in 0..positions.len()-1 {
        positions[i+1] += positions[i];
    }

    // TODO: improve the below condition
    if positions.first() == Some(&input.len()) {
        recursive_sort_u32_if_necessary(input, output, positions, pass, need_copy == false);
    } else {
        for i in (0..input.len()).rev() {
            let key: usize = ((input[i] >> (pass * 8)) as u8) as usize;
            output[positions[key]-1] = input[i];
            positions[key] -= 1;
        }

        recursive_sort_u32_if_necessary(input, output, positions, pass, need_copy);
    }
}

fn recursive_sort_u32_if_necessary(input: &mut Vec<u32>,
                            output: &mut Vec<u32>,
                            positions: &mut Vec<usize>,
                            pass: u16,
                            need_copy: bool) {
    if pass < 3 {
        unsafe {
            write_bytes(positions.as_mut_ptr(), 0, positions.len());
        }
        sort_u32(output, input, positions, pass+1, need_copy);
    } else {
        // do nothing
        if need_copy {
            output.clone_from(input);
        }
    }
}

fn sort_f32(input: &mut Vec<f32>,
    output: &mut Vec<f32>,
    positions: &mut Vec<usize>,
    pass: u16,
    need_copy: bool) {

    // Count
    unsafe {
        for n in input.iter() {
            let mut t = *n;
            let key: usize = shift_left(&mut t as *mut _ as *mut libc::c_void, pass) as usize;
            positions[key] += 1;
        }
    }

    // Calculate positions
    for i in 0..positions.len()-1 {
        positions[i+1] += positions[i];
    }

    // TODO: improve the below condition
    if positions.first() == Some(&input.len()) {
        recursive_sort_f32_if_necessary(input, output, positions, pass, need_copy == false);
    } else {
        for i in (0..input.len()).rev() {
            unsafe {
                let mut t = input[i];
                let key: usize = shift_left(&mut t as *mut _ as *mut libc::c_void, pass) as usize;
                output[positions[key]-1] = input[i];
                positions[key] -= 1;
            }
        }

        recursive_sort_f32_if_necessary(input, output, positions, pass, need_copy);
    }
}

fn recursive_sort_f32_if_necessary(input: &mut Vec<f32>,
                            output: &mut Vec<f32>,
                            positions: &mut Vec<usize>,
                            pass: u16,
                            need_copy: bool) {
    if pass < 3 {
        unsafe {
            write_bytes(positions.as_mut_ptr(), 0, positions.len());
        }
        sort_f32(output, input, positions, pass+1, need_copy);
    } else {
        // do nothing
        if need_copy {
            output.clone_from(input);
        }
    }
}

fn sort_str<'a>(input: &mut Vec<&'a String>,
    output: &mut Vec<&'a String>,
    positions: &mut Vec<usize>,
    pass: u16,
    need_copy: bool) {

    println!("pass: {}", pass);

    // Count
    unsafe {
        for n in input.iter() {
            let mut t:String = (**n).clone();
            println!("t: {}", t);
            let key: usize = shift_right(&mut t as *mut _ as *mut libc::c_void, pass) as usize;
            positions[key] += 1;
        }
    }

    // Calculate positions
    for i in 0..positions.len()-1 {
        positions[i+1] += positions[i];
    }

    // TODO: improve the below condition
    if positions.first() == Some(&input.len()) {
        recursive_sort_str_if_necessary(input, output, positions, pass, need_copy == false);
    } else {
        for n in input.iter().rev() {
            unsafe {
                let mut t:String = (**n).clone();
                println!("t: {}", t);
                let key: usize = shift_right(&mut t as *mut _ as *mut libc::c_void, pass) as usize;
                output[positions[key]-1] = n;
                positions[key] -= 1;
            }
        }

        recursive_sort_str_if_necessary(input, output, positions, pass, need_copy);
    }
}

fn recursive_sort_str_if_necessary<'a>(input: &mut Vec<&'a String>,
                            output: &mut Vec<&'a String>,
                            positions: &mut Vec<usize>,
                            pass: u16,
                            need_copy: bool) {
    if pass > 0 {
        unsafe {
            write_bytes(positions.as_mut_ptr(), 0, positions.len());
        }
        sort_str(output, input, positions, pass-1, need_copy);
    } else {
        // do nothing
        if need_copy {
            output.clone_from(input);
        }
    }
}
