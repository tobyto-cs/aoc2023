use tracing::{instrument, event, Level};


const PT_1: &'static str = include_str!("../data/day6p1.txt");
const EX_1: &'static str = include_str!("../data/day6ex1.txt");


#[instrument(skip(s))]
fn parse_nums(s: &str) -> Option<Vec<i64>> {
    s.split_once(':')
        .and_then(|(_, nums)| {
            Some(nums.trim().split(' ').into_iter()
                .filter_map(|num| { 
                    num.parse::<i64>().ok()
                }).collect::<Vec<i64>>())
        })
}

#[instrument(skip(line))]
fn parse_num(line: &str) -> Option<i64> {
    line.split_once(':')
        .and_then(|(_, nums)| {
            let str = nums.chars()
                 .filter(|n| n.is_digit(10)).collect::<String>();
            let num = str.parse::<i64>().ok()?;
            return Some(num);
        })
}

fn get_winning_races(t: i64, d: i64) -> i64 {
    let mut acc: i64 = 0;
    // 0-7
    for hold in 0..=t {
        let diff: i64 = t - hold;
        if hold * diff > d {
            acc += 1;
        } 
    }

    event!(Level::DEBUG, "race result={}", acc);
    return acc;
}

#[instrument(skip(s))]
fn solve_1(s: &'static str) -> Option<i64> {
    let mut iter = s.lines().into_iter();

    let times: Vec<i64> = iter.next().and_then(parse_nums)?;
    let distance: Vec<i64> = iter.next().and_then(parse_nums)?;
    if times.len() != distance.len() { return None }

    // (time, distance)
    let races = times.into_iter().zip(distance.into_iter());
    return Some(races.into_iter().map(|(t, d)| -> i64 {
        get_winning_races(t, d)
    }).product::<i64>());
}

#[instrument(skip(s))]
fn solve_2(s: &'static str) -> Option<i64> {
    let mut iter = s.lines().into_iter();

    let times: i64 = iter.next().and_then(parse_num)?;
    let distance: i64 = iter.next().and_then(parse_num)?;

    event!(Level::DEBUG, "time={}, distance={}", times, distance);
    return Some(get_winning_races(times, distance));
}


pub fn ex_1() -> i64 {
    return solve_1(EX_1).unwrap();
}

pub fn pt_1() -> i64 {
    return solve_1(PT_1).unwrap();
}

pub fn ex_2() -> i64 {
    return solve_2(EX_1).unwrap();
}

pub fn pt_2() -> i64 {
    return solve_2(PT_1).unwrap();
}
