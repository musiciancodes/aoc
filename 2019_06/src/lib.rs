extern crate itertools;
use itertools::Itertools;
use std::collections::HashMap;

pub fn build_tree(s: &str) -> i32 {
    let mut orbits: Vec<(&str, &str)> = s
        .lines()
        .map(|line| line.split(")").next_tuple().unwrap())
        .collect();

    let mut tree: HashMap<&str, Vec<&str>> = HashMap::new();
    orbits.iter_mut().for_each(|orb| {
        let empty_vec = Vec::new();
        let children = tree.entry(orb.0).or_insert(empty_vec);
        children.push(orb.1);
    });

    let mut count = 0;
    let mut level = 1;
    let mut to_search: Vec<&str> = vec!["COM"];
    while to_search.len() != 0 {
        let mut children: Vec<&str> = Vec::new();
        // add children to a vector
        to_search.iter().for_each(|p| {
            print!("p:{}\n", p);
            let kids = tree.get(p);
            match kids {
                Some(kids) => kids.iter().for_each(|c| {
                    children.push(c);
                    count += level;
                }),
                None => (),
            };
        });
        to_search = children;
        level += 1;
    }
    // traverse the tree:

    return count;
}

pub fn part_1(s: &str) -> i32 {
    return build_tree(s);
}

pub fn part_2(s: &str) -> i32 {
    return 0;
}
