use std::convert::TryInto;
// setup tests for the library function
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn run_opcodes(arr: Vec<i32>) -> Vec<i32> {
    let mut nums = arr;
    let mut ind = 0;
    loop {
        let next = nums[ind];
        match next {
            1 => {
                let i: usize = nums[ind + 3].try_into().unwrap();
                let a: usize = nums[ind + 1].try_into().unwrap();
                let b: usize = nums[ind + 2].try_into().unwrap();
                nums[i] = nums[a] + nums[b];
                ind += 4;
            }
            2 => {
                let i: usize = nums[ind + 3].try_into().unwrap();
                let a: usize = nums[ind + 1].try_into().unwrap();
                let b: usize = nums[ind + 2].try_into().unwrap();
                nums[i] = nums[a] * nums[b];
                ind += 4;
            }
            99 => {
                break;
            }
            _ => {
                panic!("Opcode failure")
            }
        }
    }
    return nums;
}
pub fn part_1(s: &str) -> i32 {
    let mut nums: Vec<i32> = s.split(",").map(|n| n.parse().unwrap()).collect();
    nums[1] = 12;
    nums[2] = 2;

    return run_opcodes(nums)[0];
}


pub fn part_2(s: &str) -> (i32, i32) {
    let src: Vec<i32> = s.split(",").map(|n| n.parse().unwrap()).collect();
    let mut dst;
    for noun in 0..100 {
        for verb in 0..100 {
            dst = src.clone();
            dst[1] = noun;
            dst[2] = verb;
            if run_opcodes(dst)[0] == 19690720 {
                return (noun, verb);
            }
        }
    }
    panic!("Could not complete Part 2!");
}
