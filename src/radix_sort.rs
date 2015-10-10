extern crate rlibc;

use sort::Sort;

pub struct RadixSort {
    name: String,
}

impl RadixSort {
    fn sort_<'a>(&'a self,
                 input: &'a mut [u32],
                 output: &'a mut [u32],
                 positions: &mut [usize],
                 level: u16) -> &[u32] {
        // Count
        for n in input.iter() {
            let key: usize = ((n >> (level * 8)) as u8) as usize;
            positions[key] += 1;
        }

        // Calculate positions
        for i in 0..positions.len()-1 {
            positions[i+1] += positions[i];
        }

        if positions.first() == Some(&input.len()) {
            return self.recursive_call_or_return(input, output, positions, level)
        }

        for i in (0..input.len()).rev() {
            let key: usize = ((input[i] >> (level * 8)) as u8) as usize;
            output[positions[key]-1] = input[i];
            positions[key] -= 1;
        }

        self.recursive_call_or_return(input, output, positions, level)
    }

    fn recursive_call_or_return<'a>(&'a self,
                                    input: &'a mut [u32],
                                    output: &'a mut [u32],
                                    positions: &mut [usize],
                                    level: u16) -> &[u32] {
        if level < 3 {
            //unsafe { rlibc::memset(positions.as_mut_ptr() as *mut u8, 0, positions.len()) };
            for i in 0..positions.len() {
                positions[i] = 0;
            }
            self.sort_(output, input, positions, level+1)
        } else {
            // do nothing
            input
        }
    }
}

impl Sort for RadixSort {
    fn new() -> RadixSort {
        RadixSort {
            name: "Radix sort".to_string(),
        }
    }
    
    fn name(&self) -> &String {
        &self.name
    }

    fn sort(&self, numbers: &mut Vec<u32>) {
        let mut clone: Vec<u32> = numbers.clone();
        let mut tmp: Vec<u32> = vec![0; numbers.len()];
        let mut positions: Vec<usize> = vec![0; 256];
        let sorted: &[u32] = self.sort_(clone.as_mut_slice(),
                                        tmp.as_mut_slice(),
                                        positions.as_mut_slice(),
                                        0);
        numbers.clear();
        numbers.push_all(sorted);
    }
}
