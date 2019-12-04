
use std::convert::TryInto;
#[test]
fn first() {
    assert_eq!(check_criteria(111111), true);
}
#[test]

fn second() {
    assert_eq!(check_criteria(223450), false);
}
#[test]

fn third() {
    assert_eq!(check_criteria(123789), false);
}

#[test]
fn t2_1() {
    assert_eq!(check_criteria_2(112233), true);
}
#[test]

fn t2_2() {
    assert_eq!(check_criteria_2(123444), false);
}
#[test]

fn t2_3() {
    assert_eq!(check_criteria_2(111122), true);
}

pub fn check_criteria(i: i32) -> bool {
    let i_str = i.to_string();
    let mut i: i32 = 0;
    for c in i_str.chars() {
        let x:i32 = c.to_digit(10).unwrap().try_into().unwrap();
        if x < i {
            return false;
        }
        else {
            i = x;
        }
    }

    let mut adj:i32 = 0;
    for c in i_str.chars() {
        let x:i32 = c.to_digit(10).unwrap().try_into().unwrap();
        if x == adj {
            return true
        }
        else {
            adj = x;
        }
    }
    return false;
}

pub fn check_criteria_2(i: i32) -> bool {
    let i_str = i.to_string();
    let mut i: i32 = 0;
    for c in i_str.chars() {
        let x:i32 = c.to_digit(10).unwrap().try_into().unwrap();
        if x < i {
            return false;
        }
        else {
            i = x;
        }
    }

    let mut adj:i32 = 1;
    let mut adj_count:i32 = 0;
    for c in i_str.chars() {
        let x:i32 = c.to_digit(10).unwrap().try_into().unwrap();
        if x == adj {
            adj_count += 1;
        }
        else {
            if adj_count == 2 {return true};
            adj_count = 1;
            adj = x;
        }
    }
    if adj_count == 2 {return true};
    return false;
}

pub fn part_2 (s: &str) -> i32 {
    let range: Vec<i32> = s.split("-").map(|s| s.parse().unwrap()).collect();
    let mut count = 0;
    for x in range[0]..range[1]{
        if check_criteria_2(x){count += 1}
    }
    return count;

}

pub fn part_1 (s: &str) -> i32 {
    let range: Vec<i32> = s.split("-").map(|s| s.parse().unwrap()).collect();
    let mut count = 0;
    for x in range[0]..range[1]{
        if check_criteria(x){count += 1}
    }
    return count;

}