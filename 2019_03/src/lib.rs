extern crate itertools;
use itertools::Itertools;
use std::collections::{HashSet,HashMap};
use std::iter::FromIterator;
type PointMap = HashMap<(i32, i32), i32>;


pub fn points_with_step(s: &str) -> PointMap {
    let mut points = PointMap::new();
    let mut loc = (0, 0);
    let mut steps = 0;
    s.split(",").for_each(|s| {
        let cardinal = s.chars().nth(0).unwrap();
        let mag: i32 = s.get(1..).unwrap().parse().unwrap();
        match cardinal {
            'R' => {
                for _i in 0..mag {
                    loc.0 += 1;
                    steps +=1;
                    points.entry(loc).or_insert(steps);
                }
            }
            'L' => {
                for _i in 0..mag {
                    loc.0 -= 1;
                    steps +=1;
                    points.entry(loc).or_insert(steps);
                }
            }
            'U' => {
                for _i in 0..mag {
                    loc.1 += 1;
                    steps +=1;
                    points.entry(loc).or_insert(steps);
                }
            }
            'D' => {
                for _i in 0..mag {
                    loc.1 -= 1;
                    steps +=1;
                    points.entry(loc).or_insert(steps);
                }
            }
            _ => panic!("Unable to match cardinal direction!"),
        }
    });
    points
}

pub fn part_2(s: &str) -> i32 {
    let (l1, l2) = s.lines().next_tuple().unwrap();
    let pm1 = points_with_step(l1);
    let pm2 = points_with_step(l2);
    let k1s: HashSet<&(i32, i32)> = HashSet::from_iter(pm1.keys());
    let k2s: HashSet<&(i32, i32)> = HashSet::from_iter(pm2.keys());
    let intersection = k1s.intersection(&k2s);
    let steps: Vec<i32> = intersection
        .map(|a| pm1.get(a).unwrap() + pm2.get(a).unwrap())
        .collect();
    *steps.iter().min().unwrap()
}

pub fn line_points(s: &str) -> HashSet<(i32, i32)> {
    let mut points: HashSet<(i32, i32)> = HashSet::new();
    let mut loc = (0, 0);
    s.split(",").for_each(|s| {
        let cardinal = s.chars().nth(0).unwrap();
        let mag: i32 = s.get(1..).unwrap().parse().unwrap();
        match cardinal {
            'R' => {
                for _i in 0..mag {
                    loc.0 += 1;
                    points.insert(loc);
                }
            }
            'L' => {
                for _i in 0..mag {
                    loc.0 -= 1;
                    points.insert(loc);
                }
            }
            'U' => {
                for _i in 0..mag {
                    loc.1 += 1;
                    points.insert(loc);
                }
            }
            'D' => {
                for _i in 0..mag {
                    loc.1 -= 1;
                    points.insert(loc);
                }
            }
            _ => panic!("Unable to match cardinal direction!"),
        }
    });
    points
}

pub fn part_1(s: &str) -> i32 {
    let (l1, l2) = s.lines().next_tuple().unwrap();
    let points1 = line_points(l1);
    let points2 = line_points(l2);
    let intersection = points1.intersection(&points2);
    let distances: Vec<i32> = intersection
        .map(|coords| coords.0.abs() + coords.1.abs())
        .collect();
    *distances.iter().min().unwrap()
}
