
use std::{collections::{HashSet, HashMap}, ops::Add};

use tracing::{instrument, event, Level};

const PT_1: &'static str = include_str!("../data/day4p1.txt");
const EX_1: &'static str = include_str!("../data/day4ex1.txt");

fn parse_nums(s: Option<&str>) -> Option<HashSet<u32>> {
    s.and_then(|st|
        Some(st.trim().split(' ')
            .filter_map(|num| num.parse::<u32>().ok())
            .collect::<HashSet<u32>>()))
}

fn parse_id(s: Option<&str>) -> Option<u32> {
    s.and_then(|st| st.trim().split_whitespace().nth(1)
               .and_then(|id| id.trim().parse::<u32>().ok()))
}

fn matching(a: HashSet<u32>, b: HashSet<u32>) -> Option<u32> {
    a.intersection(&b).count().try_into().ok()
}

#[instrument(skip(s))]
fn solve_1(s: &'static str) -> u32 {
    // split into lines and by ':'
    return s.split(&[':', '\n'])
        .filter_map(|line| {
            // Cut out all 'Card #' string
            if line.contains("Card") { return None } 
            else { return Some(line)}
        }).map(|card| -> Option<u32> {
            // 1 2 3 4 5 | 8 7 6 5 4 3 2 1
            let mut iter = card.split('|');

            let winning_nums: HashSet<u32> = parse_nums(iter.next())?;
            let nums: HashSet<u32> = parse_nums(iter.next())?;
            
            return matching(winning_nums, nums)
                .and_then(|n: u32| n.checked_sub(1)
                          .and_then(|exp| u32::checked_pow(2, exp)));
        }).filter_map(|num| num).sum::<u32>();
}

#[instrument(skip(s))]
fn solve_2(s: &'static str) -> u32 {
    // split into lines and by ':'
    return s.lines().map(|card| {
            // 1 2 3 4 5 | 8 7 6 5 4 3 2 1
            let mut iter = card.split(&[':', '|']);

            let card_id: u32 = parse_id(iter.next())?; 
            let winning_nums: HashSet<u32> = parse_nums(iter.next())?;
            let nums: HashSet<u32> = parse_nums(iter.next())?;

            return Some((card_id, matching(winning_nums, nums)?));
    }).filter_map(|a| a)
    .fold(HashMap::new(), |mut hmap: HashMap<u32, u32>, (id, mcount)|  {

        if let Some(count) = hmap.get(&id) {
            hmap.insert(id, count+1);
        } else {
            hmap.insert(id, 1);
        }
        let num_card = *hmap.get(&id).unwrap();
        event!(Level::DEBUG, "id={}, matches={}, copies={}", id, mcount, num_card);
        if mcount == 0 { return hmap }

        for i in id+1..=id+mcount {
            event!(Level::DEBUG, "  adding {} to card {}", num_card, i);
            if let Some(count) = hmap.get(&i) {
                hmap.insert(i, count+num_card);
            } else {
                hmap.insert(i, num_card);
            }
        }
        event!(Level::DEBUG, "  {:?}", hmap);
        return hmap;
    }).iter().fold(0, |a, (_, v)| v + a);
}

pub fn pt_1() -> u32 {
    return solve_1(PT_1);
}

pub fn pt_2() -> u32 {
    return solve_2(PT_1);
}

pub fn ex_1() -> u32 {
    return solve_1(EX_1);
}

pub fn ex_2() -> u32 {
    return solve_2(EX_1);
}
