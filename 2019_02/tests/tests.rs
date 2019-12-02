use aoc::run_opcodes;

#[cfg(test)]
mod part1_tests {
    use super::*;
    #[test]
    fn first() {
        let input: Vec<i32> = String::from("1,0,0,0,99").split(",").map(|n| n.parse().unwrap()).collect();
        let output: Vec<i32> = vec![2, 0, 0, 0, 99];
        assert_eq!(run_opcodes(input), output);
    }
    #[test]
    fn second() {
        let input: Vec<i32> = String::from("2,3,0,3,99").split(",").map(|n| n.parse().unwrap()).collect();
        let output: Vec<i32> = vec![2,3,0,6,99];
        assert_eq!(run_opcodes(input), output);
    }    
    #[test]
    fn third() {
        let input: Vec<i32> = String::from("2,4,4,5,99,0").split(",").map(|n| n.parse().unwrap()).collect();
        let output: Vec<i32> = vec![2,4,4,5,99,9801];
        assert_eq!(run_opcodes(input), output);
    }        
    #[test]
    fn fourth() {
        let input: Vec<i32> = String::from("1,1,1,4,99,5,6,0,99").split(",").map(|n| n.parse().unwrap()).collect();
        let output: Vec<i32> = vec![30,1,1,4,2,5,6,0,99];
        assert_eq!(run_opcodes(input), output);
    }            
}

#[cfg(test)]
mod part2_tests {}
