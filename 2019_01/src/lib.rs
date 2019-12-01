// setup tests for the library function
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn module_mass(s: &str) -> i64 {
    let num: i64 = s.parse().unwrap();
    return num / 3 - 2;
}

pub fn module_mass_max(s: &str) -> i64 {
    let mut sum: i64 = 0;
    let mut num = module_mass(s);
    loop {
        if num > 0 {
            sum += num;
            num = num/3 - 2;
        } else {
            break;
        }
    }
    return sum;
}

pub fn part_1(s: &str) -> i64 {
    let mut sum = 0;
    for l in s.lines() {
        sum += module_mass(l);
    }
    return sum;
}

pub fn part_2(s: &str) -> i64 {
    let mut sum = 0;
    for l in s.lines() {
        sum += module_mass_max(l);
    }
    return sum;
}
