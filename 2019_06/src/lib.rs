
extern crate itertools;
use itertools::Itertools;
use std::collections::{HashMap};


pub fn build_tree(s: &str) -> i32 {

    let mut orbits: Vec<(&str, &str)> = s.lines().map(|line| {
        line.split(")").next_tuple().unwrap()
    }).collect();

    let mut tree: HashMap<&str, Vec<&str>> = HashMap::new();
    orbits.iter_mut().for_each(|orb| {
        let empty_vec = Vec::new();
        let children = tree.entry(orb.0).or_insert(empty_vec);
        children.push(orb.1);
    });

    let mut count = 0;
    let mut level = 0;
    print!("{:?}\n", tree);
    
    //     let v: Vec<&str> = line.split(")").collect();
    //     orbits.entry(v[0]).or_insert(Vec::new());
    //     orbits.get(v[0]).unwrap().push(v[1]);
    
    return 0;
}

pub fn part_1(s: &str) -> i32 {
    build_tree(s);
    return 0;
}

pub fn part_2(s: &str) -> i32 {
    return 0;
}
