use std::fs::File;
use std::io::{Read};
use aoc::{part_1};


fn main() {
    let path: String = String::from("input.txt");
    let mut input = File::open(path).unwrap();
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    let solution = part_1(&buffer);
    println!("Part 1: {}", solution);    
}