use aoc::{part_1};

#[cfg(test)]
mod part1_tests {
    use super::*;    
    #[test]
    fn first() {
        let input = String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(part_1(&input), 159);
    }
}

// #[cfg(test)]
// mod part2_tests {
//     use super::*;    
//     #[test]
//     fn first() {
//         let input = String::from("14");
//         assert_eq!(1+1, 2);
//     }
// }