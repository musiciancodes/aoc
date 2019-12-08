use std::convert::TryInto;
extern crate itertools;
use itertools::Itertools;

#[derive(Clone)]
pub struct Computer {
    loc: usize,
    instructions: Vec<i32>,
    inputs: Vec<i32>,
    outputs: Vec<i32>,
    input_idx: usize,
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
        let x = self.inputs[self.input_idx];
        let i: usize = self.get_ix_u(self.loc + 1);
        self.input_idx +=1;
        self.update_ins(i, x);
        self.update_loc(2)
    }
    fn op_4(&mut self, p_modes: Vec<bool>) {
        let a: i32 = self.get_val(self.loc + 1, p_modes[2]);
        self.outputs.push(a);
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

fn compute_part2(s: &str, permutation: Vec<i32>) -> i32 {
    let nums: Vec<i32> = s.split(",").map(|n| n.parse().unwrap()).collect();    

    let mut amps = vec![
        Computer {
            loc: 0,
            instructions: nums.clone(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            input_idx: 0
        };
        5
    ];

    fn pass_input(amp: &mut Computer, value: i32) {
        amp.inputs.insert(0, value);
    }

    for (i, &phase) in permutation.iter().enumerate() {
        pass_input(&mut amps[i], phase);
    }
    // start the first input
    pass_input(&mut amps[0], 0);

    loop {
        let mut had_progress = false;
        for idx in 0..5 {
            let amp = &mut amps[idx];
            amp.run();
            if amp.outputs.is_empty() {
                continue;
            }
            
            had_progress = true;
            let next_idx = (idx + 1) % 5;
            for out_idx in 0..amp.outputs.len() {
                let value = amps[idx].outputs[out_idx];
                pass_input(&mut amps[next_idx], value);
            }
            amps[idx].outputs.clear();
        }

        if !had_progress {
            if amps[0].inputs.len() != 1 || amps.iter().skip(1).any(|amp| amp.inputs.len() != 0) {
                break("incorrect passing of arguments between amplifiers");
            }
        }
    };
    return amps[0].inputs[0];
}

pub fn part_2(s:&str) -> i32{

    let mut max_signal:i32 = 0;
    (5..10).permutations(5).for_each(|input|{
        let signal = compute_part2(s, input);
        print!("signal, {}\n", signal);
        if signal > max_signal {max_signal = signal};
    });
    return max_signal;    
}