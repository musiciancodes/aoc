use std::convert::TryInto;
// setup tests for the library function
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first() {
        let input = String::from("1002,4,3,4,33");
        part_1(&input);
    }      
}

pub struct Computer {
    loc: usize,
    instructions: Vec<i32>,
}

impl Computer {
    pub fn run(&mut self) {
        loop {
            let mut ins = self.instructions[self.loc].to_string();
            while ins.chars().count() < 5{
                ins = format!("0{}", ins);
            }
            let opcode:i32 = ins[3..].parse().unwrap();
            let p_mode: Vec<bool> = ins[..3].chars().map(|x| x == '0').collect();
            match opcode {
                1 => self.op_1(p_mode),
                2 => self.op_2(p_mode),
                3 => self.op_3(1),
                4 => self.op_4(p_mode),
                99 => break,
                _ => panic!("Bad instruction match {}\n", opcode),
            };
        }
    }
    fn update_loc(&mut self, shift: usize) {
        self.loc += shift
    }
    fn update_ins(&mut self, loc: usize, val: i32) {
        self.instructions[loc] = val
    }
    fn get_ix(&mut self, i: usize) -> i32{
        return self.instructions[i];
    }
    fn get_ix_u(&mut self, i: usize) -> usize{
        return self.instructions[i].try_into().unwrap();
    }
    fn get_val(&mut self, i:usize, pmode: bool)-> i32{
        if pmode {
            let pos = self.get_ix_u(i);
            return self.get_ix(pos);
        }
        return self.get_ix(i);
    }
    fn op_1(&mut self, p_modes: Vec<bool>) {
        let i: usize = self.get_ix_u(self.loc + 3);
        let a = self.get_val(self.loc + 1, p_modes[2]);
        let b = self.get_val(self.loc + 2, p_modes[1]);
        self.update_ins(i, a + b);
        self.update_loc(4)
    }
    fn op_2(&mut self, p_modes: Vec<bool>) {
        let i: usize = self.get_ix_u(self.loc + 3);
        let a = self.get_val(self.loc + 1, p_modes[2]);
        let b = self.get_val(self.loc + 2, p_modes[1]);
        self.update_ins(i, a * b);
        self.update_loc(4)
    }
    fn op_3(&mut self, x: i32) {
        let a: usize = self.get_ix_u(self.loc + 1);
        self.update_ins(a, x);
        self.update_loc(2)
    }
    fn op_4(&mut self, p_modes: Vec<bool>) {
        let a:i32  = self.get_val(self.loc + 1, p_modes[2]);
        print!("op 4: {}\n", a);
        self.update_loc(2)
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