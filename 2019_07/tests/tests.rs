use aoc::{part_1};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_equality() {
        let s = String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        assert_eq!(part_1(&s), 43210);
    }
//     #[test]
//     fn test_less_than() {
//         let s = String::from("3,9,7,9,10,9,4,9,99,-1,8");
//         let input = vec![5];
//         assert_eq!(test_signal(&s, input), 1);
//     }  
//     #[test]
//     fn test_eq_immediate() {
//         let s = String::from("3,3,1108,-1,8,3,4,3,99");
//         let input = vec![8];
//         assert_eq!(test_signal(&s, input), 1);
//     }  
//     #[test]
//     fn test_less_than_immediate() {
//         let s = String::from("3,3,1107,-1,8,3,4,3,99");
//         let input = vec![5];
//         assert_eq!(test_signal(&s, input), 1);
//     }            
}