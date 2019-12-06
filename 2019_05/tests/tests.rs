use aoc::{part_1};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_equality() {
        let input = String::from("3,9,8,9,10,9,4,9,99,-1,8");
        assert_eq!(part_1(&input, 8), 1);
    }
    #[test]
    fn test_less_than() {
        let input = String::from("3,9,7,9,10,9,4,9,99,-1,8");
        assert_eq!(part_1(&input, 5), 1);
    }  
    #[test]
    fn test_eq_immediate() {
        let input = String::from("3,3,1108,-1,8,3,4,3,99");
        assert_eq!(part_1(&input, 8), 1);
    }  
    #[test]
    fn test_less_than_immediate() {
        let input = String::from("3,3,1107,-1,8,3,4,3,99");
        assert_eq!(part_1(&input, 5), 1);
    }            
}