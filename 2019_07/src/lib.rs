use std::convert::TryInto;
extern crate itertools;
use itertools::Itertools;
// extern crate combinations;
// use combinations::Combinations;

pub struct Computer {
    loc: usize,
    instructions: Vec<i32>,
    phase: i32,
    input_signal: i32,
    output: i32,
}

impl Computer {
    pub fn run(&mut self) {
        loop {
            let mut ins = self.instructions[self.loc].to_string();
            while ins.chars().count() < 5 {
                ins = format!("0{}", ins);
            }
            let opcode: i32 = ins[3..].parse().unwrap();
            let p_mode: Vec<bool> = ins[..3].chars().map(|x| x == '0').collect();
            match opcode {
                1 => self.op_1(p_mode),
                2 => self.op_2(p_mode),
                3 => self.op_3(),
                4 => self.op_4(p_mode),
                5 => self.op_5(p_mode),
                6 => self.op_6(p_mode),
                7 => self.op_7(p_mode),
                8 => self.op_8(p_mode),
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
    fn get_ix(&mut self, i: usize) -> i32 {
        return self.instructions[i];
    }
    fn get_ix_u(&mut self, i: usize) -> usize {
        return self.instructions[i].try_into().unwrap();
    }
    fn get_val(&mut self, i: usize, pmode: bool) -> i32 {
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
    fn op_3(&mut self) {
        let x = self.phase;
        let i: usize = self.get_ix_u(self.loc + 1);
        self.update_ins(i, x);
        self.phase = self.input_signal;
        self.update_loc(2)
    }
    fn op_4(&mut self, p_modes: Vec<bool>) {
        let a: i32 = self.get_val(self.loc + 1, p_modes[2]);
        self.output = a;
        print!("op4: {}\n", a);        
        self.update_loc(2)
    }
    fn op_5(&mut self, p_modes: Vec<bool>) {
        let jump: bool = self.get_val(self.loc + 1, p_modes[2]) != 0;
        let new_loc: usize = self.get_val(self.loc + 2, p_modes[1]).try_into().unwrap();
        if jump {
            self.loc = new_loc
        } else {
            self.update_loc(3)
        };
    }
    fn op_6(&mut self, p_modes: Vec<bool>) {
        let jump: bool = self.get_val(self.loc + 1, p_modes[2]) == 0;
        let new_loc: usize = self.get_val(self.loc + 2, p_modes[1]).try_into().unwrap();
        if jump {
            self.loc = new_loc
        } else {
            self.update_loc(3)
        }
    }
    fn op_7(&mut self, p_modes: Vec<bool>) {
        // less than
        let a = self.get_val(self.loc + 1, p_modes[2]);
        let b = self.get_val(self.loc + 2, p_modes[1]);
        let i: usize = self.get_ix_u(self.loc + 3);
        if a < b {
            self.update_ins(i, 1);
        } else {
            self.update_ins(i, 0);
        }
        self.update_loc(4)
    }
    fn op_8(&mut self, p_modes: Vec<bool>) {
        // equals
        let a = self.get_val(self.loc + 1, p_modes[2]);
        let b = self.get_val(self.loc + 2, p_modes[1]);
        let i: usize = self.get_ix_u(self.loc + 3);
        if a == b {
            self.update_ins(i, 1);
        } else {
            self.update_ins(i, 0);
        }
        self.update_loc(4)
    }
}

pub fn part_1(s: &str) -> i32 {
    let mut max_signal:i32 = 0;
    (0..5).permutations(5).for_each(|input|{
        let signal = test_sequence(s, input);
        if signal > max_signal {max_signal = signal};
    });
    return max_signal;
}

pub fn test_sequence(s:&str, phases:Vec<i32>) -> i32 {
    let mut input_signal = 0;
    phases.iter().for_each(|phase| {
        input_signal = test_signal(s, *phase, input_signal);
    });
    return input_signal;
}
pub fn test_signal(s:&str, phase:i32, input_signal: i32) -> i32 {
    let nums: Vec<i32> = s.split(",").map(|n| n.parse().unwrap()).collect();
    let mut comp = Computer {
        loc: 0,
        instructions: nums,
        phase: phase,
        input_signal: input_signal,
        output:0
    };
    comp.run();
    return comp.output;
}