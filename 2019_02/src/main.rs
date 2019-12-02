use std::fs::File;
use std::io::{Read};
use aoc::{part_1, part_2};


fn run(path: &str) {
    let mut input = File::open(path).unwrap();
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    let solution = part_1(&buffer);
    println!("Part 1: {}", solution);
    input.read_to_string(&mut buffer).unwrap();
    let sol2 = part_2(&buffer);
    println!("Part 2: {}", sol2[0]*100 + sol2[1]);    
}

fn main() {
    let path: String = String::from("input.txt");
    run(&path);
}
