const EX_1: &'static str = include_str!("../data/day1ex1.txt");
const EX_2: &'static str = include_str!("../data/day1ex2.txt");
const PT_1: &'static str = include_str!("../data/day1p1.txt");

const SPELLED: [(&str, char); 9]  = [ 
    ("one", '1'), ("two", '2'), ("three", '3'), 
    ("four", '4'), ("five", '5'), ("six", '6'), 
    ("seven", '7'), ("eight", '8'), ("nine", '9')
];

const VALID_MATCHES: [&str; 18] = [
    "one", "1", "two", "2", "three", "3", 
    "four", "4", "five", "5", "six", "6", 
    "seven", "7", "eight", "8", "nine", "9"
];

#[cfg(test)]
mod tests {
    use crate::day1;

    #[test]
    fn example_1() {
        assert_eq!(day1::ex_1(), 142);
    }

    #[test]
    fn example_2() {
        assert_eq!(day1::ex_2(), 281)
    }
}

fn convert_digit(s: &str) -> char {
    if s.parse::<u32>().is_ok() {
        return s.chars().next().unwrap();
    }

    for (k, v) in SPELLED {
        if s == k {
            return v;
        }
    }
    return ' ';
}

fn solve(s:&'static str) -> u32 {
    return s.lines().map(|line: &str| -> u32 {
        let mut res = Vec::new();

        for pattern in VALID_MATCHES {
            res.extend(line.match_indices(pattern)); 
        } 

        let (_, min) = res.clone().into_iter().min_by(|(ia, _), (ib, _)| { 
            ia.partial_cmp(ib).unwrap() }).unwrap();
        let (_, max) = res.into_iter().max_by(|(ia, _), (ib, _)| { 
            ia.partial_cmp(ib).unwrap() }).unwrap();

        return format!("{}{}", convert_digit(min), convert_digit(max)).parse::<u32>().unwrap();
    }).sum::<u32>();
}

#[allow(dead_code)]
pub fn ex_1() -> u32 {
    return solve(EX_1);
}

#[allow(dead_code)]
pub fn ex_2() -> u32 {
    return solve(EX_2);
}

pub fn pt_1() -> u32 {
    return solve(PT_1);
}


