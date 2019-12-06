use std::collections::{HashMap};

pub fn build_tree(s: &str) -> i32 {
    // HashSet: parent: [children] (use or_insert) 
    //
    let mut orbits: HashMap<&str,  Vec<&str>>;
    s.lines().for_each(|line| {
        let v: Vec<&str> = line.split(")").collect();
        orbits.entry(v[0]).or_insert(Vec::new());
        //orbits.get(v[0]).unwrap().push(v[1]);
    });

    return 0;
}

pub fn part_1(s: &str) -> i32 {
    return 0;
}

pub fn part_2(s: &str) -> i32 {
    return 0;
}
