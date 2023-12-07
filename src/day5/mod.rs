#![feature(alloc)]

use std::ops::Range;

use nom::{IResult, 
    bytes::{complete::tag, streaming::take_till}, 
    Parser, 
    sequence::{preceded, tuple}, 
    multi::{separated_list1, many_till, many1}, 
    combinator::{recognize, map_res, not, eof, consumed}, 
    character::{complete::digit1, is_digit, is_alphabetic}, error::dbg_dmp};
use tracing::{instrument, event, Level};

const EX_1: &'static str = include_str!("../data/day5ex1.txt");

#[instrument]
fn my_i32(input : &str) -> IResult<&str, i32> {
    // event!(Level::DEBUG, "");
    map_res(recognize(digit1), str::parse::<i32>)(input)
}

#[instrument]
fn parse_seeds(i: &str) -> IResult<&str, Vec<i32>> {
    let (i, _) = tag("seeds: ")(i)?;
    return separated_list1(tag(" "), my_i32)(i);
}

#[instrument]
fn parse_map(line: &str) -> Option<(Range<i32>, i32)> {
    let vec: Vec<i32> = line.split(" ").filter_map(|num| num.parse::<i32>().ok()).collect();
    if vec.len() == 3 { Some((vec[1]..vec[1]+vec[2], vec[0]-vec[1])) } else { None }
}

#[instrument(skip(s))]
fn solve_1(s: &'static str) -> i32 {
    let (s, seeds) = parse_seeds(s).unwrap();
    event!(Level::DEBUG, "seeds={:?}", seeds);

    let maps = s.trim().split("\n\n")
        .map(|conv_map| -> Vec<(Range<i32>, i32)> {
            conv_map.trim().lines()
                .filter(|line| !line.contains("map:"))
                .filter_map(|line| parse_map(line))
                .collect()
        }).fold(seeds.clone(), |acc, vec| -> Vec<i32> {
            let tmp: Vec<i32> = acc.iter().map(|&seed| {
                vec.iter().fold(seed, |seed_acc, (rng, trns)| {
                    if rng.contains(&seed_acc) {
                        event!(Level::TRACE, "  {} is in range {:?}, returning {}", seed_acc, rng, seed_acc + trns);
                        seed_acc + trns
                    } else { seed_acc }
                })
            }).collect();
            event!(Level::INFO, "seeds={:?}, vec={:?}", tmp, vec);

            return tmp;
        });
    event!(Level::INFO, "maps={:?}", maps);

    let vec: Vec<(Range<i32>, i32)> = s.trim().lines().into_iter()
        .filter(|line| !line.is_empty() && !line.contains("map:"))
        .filter_map(parse_map).collect();

    let res = seeds.iter().map(|&seed| {
        vec.iter().fold(seed, |acc, (rng, trns)| {
            if rng.contains(&acc) {
                acc + trns
            } else { acc }
        })
    }).min();

    event!(Level::DEBUG, "{:?}", res);


    return 0;
}

pub fn ex_1() -> i32 {
    return solve_1(EX_1);
}
