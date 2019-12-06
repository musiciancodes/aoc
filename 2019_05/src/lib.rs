use std::convert::TryInto;
// setup tests for the library function
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first() {
        let input = String::from("1,0,0,0,99");
        assert_eq!(part_1(&input), 2);
    }
    #[test]
    fn second() {
        let input = String::from("1,1,1,4,99,5,6,0,99");
        assert_eq!(part_1(&input), 30);
    }
    #[test]
    fn third() {
        let input = String::from("2,4,4,5,99,0");
        assert_eq!(part_1(&input), 2);
    }  
    #[test]
    fn fourth() {
        let input = String::from("2,3,0,3,99");
        assert_eq!(part_1(&input), 2);
    }        
}

pub struct Computer {
    loc: usize,
    instructions: Vec<i32>,
}

impl Computer {
    pub fn run(&mut self) {
        self.replace();
        print!("{:?}\n", self.instructions);
        loop {
            let next = self.instructions[self.loc];
            match next {
                1 => self.op_1(),
                2 => self.op_2(),
                99 => break,
                _ => panic!("Bad instruction match {}\n", next),
            };
        }
    }
    fn replace(&mut self) {
        self.update_ins(1, 12);
        self.update_ins(2, 2);
    }
    fn update_loc(&mut self, shift: usize) {
        self.loc += shift
    }
    fn update_ins(&mut self, loc: usize, val: i32) {
        self.instructions[loc] = val
    }
    fn get_ix(&mut self, i: usize) -> i32 {
        return self.instructions[i];
    }
    fn op_1(&mut self) {
        let i: usize = self.get_ix(self.loc + 3).try_into().unwrap();
        let a_i: usize = self.get_ix(self.loc + 1).try_into().unwrap();
        let b_i: usize = self.get_ix(self.loc + 2).try_into().unwrap();
        let a = self.get_ix(a_i);
        let b = self.get_ix(b_i);
        self.update_ins(i, a + b);
        self.update_loc(4)
    }
    fn op_2(&mut self) {
        let i: usize = self.get_ix(self.loc + 3).try_into().unwrap();
        let a_i: usize = self.get_ix(self.loc + 1).try_into().unwrap();
        let b_i: usize = self.get_ix(self.loc + 2).try_into().unwrap();
        let a = self.get_ix(a_i);
        let b = self.get_ix(b_i);
        self.update_ins(i, a * b);
        self.update_loc(4)
    }
}

pub fn part_1(s: &str) -> i32 {
    let nums: Vec<i32> = s.split(",").map(|n| n.parse().unwrap()).collect();
    let mut comp = Computer {
        loc: 0,
        instructions: nums,
    };
    comp.run();
    return comp.instructions[0];
}
