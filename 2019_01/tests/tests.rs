use aoc::{module_mass, module_mass_max};

#[cfg(test)]
mod part1_tests {
    use super::*;    
    #[test]
    fn first() {
        let input = String::from("12");
        assert_eq!(module_mass(&input), 2);
    }
    #[test]
    fn second() {
        let input = String::from("14");
        assert_eq!(module_mass(&input), 2);
    }
    #[test]
    fn third() {
        let input = String::from("1969");
        assert_eq!(module_mass(&input), 654);
    }  
    #[test]
    fn fouth() {
        let input = String::from("100756");
        assert_eq!(module_mass(&input), 33583);
    }        
}

#[cfg(test)]
mod part2_tests {
    use super::*;    
    #[test]
    fn first() {
        let input = String::from("14");
        assert_eq!(module_mass_max(&input), 2);
    }
    #[test]
    fn second() {
        let input = String::from("1969");
        assert_eq!(module_mass_max(&input), 966);
    }  
    #[test]
    fn third() {
        let input = String::from("100756");
        assert_eq!(module_mass_max(&input), 50346);
    }      
}