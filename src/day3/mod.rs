const PT_1: &'static str = include_str!("../data/day3p1.txt");
const EX_1: &'static str = include_str!("../data/day3ex1.txt");

use std::{ops::{Range, RangeInclusive}, collections::HashSet};

use tracing::{instrument, event, Level};

#[instrument]
fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

#[instrument]
fn is_valid_pos(pos: (usize, usize), row_max: usize, col_max: usize) -> bool {
    let (r, c) = pos;
    event!(Level::TRACE, "{:?}", pos);
    return r < row_max && c < col_max;
}

#[instrument(skip(l))]
fn search_adj(row: usize, r: RangeInclusive<usize>, l: Vec<Vec<char>>) -> bool {

    let mut pos: HashSet<(Option<usize>, Option<usize>)> = HashSet::new();

    for col in r {
        event!(Level::TRACE, "getting pos for col={}", col);
        pos.insert((row.checked_sub(1), Some(col)));
        pos.insert((Some(row), col.checked_sub(1)));
        pos.insert((Some(row), col.checked_add(1)));
        pos.insert((row.checked_add(1), Some(col)));
        pos.insert((row.checked_sub(1), col.checked_sub(1)));
        pos.insert((row.checked_sub(1), col.checked_add(1)));
        pos.insert((row.checked_add(1), col.checked_add(1)));
        pos.insert((row.checked_add(1), col.checked_sub(1)));
    }
    event!(Level::DEBUG, "pos={:?}", pos);

    let row_max = l.len();
    let col_max = l[0].len();
    event!(Level::DEBUG, "rm={}, cm={}", row_max, col_max);
    return pos.iter()
        .filter(|(r, c)| r.is_some() && c.is_some())
        .map(|(r, c)| (r.unwrap(), c.unwrap()))
        .filter(|p| {
            let is_valid = is_valid_pos(*p, row_max, col_max);
            event!(Level::TRACE, "is_valid={}", is_valid);
            return is_valid;
        }).find(|(row, c)| { 
            let ch = l[*row][*c];
            event!(Level::TRACE, "ch={}", ch);
            return is_symbol(ch);
        }).is_some();
}

fn parse_file(s: &'static str) -> Vec<Vec<char>> {
     s.lines().map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

#[instrument(skip(l))]
fn find_symbols(l: Vec<Vec<char>>) -> Vec<(usize, usize)> {
    return l.iter().enumerate().map(|(r, chars)| -> Vec<(usize, usize)>{
        return chars.iter().enumerate().filter_map(|(c, ch)| -> Option<(usize, usize)> {
            if is_symbol(*ch) { Some((r, c)) } else { None }
        }).collect::<Vec<(usize, usize)>>();
    }).flatten()
      .collect::<Vec<(usize, usize)>>();
}


#[instrument(skip(l, symbols))]
fn find_adj_digits(l: Vec<Vec<char>>, symbols: &[(usize, usize)]) -> Vec<u32> {
    let mut res: Vec<u32> = Vec::new();
    let row_max = l.len();
    let col_max = l[0].len();

    for (row, col) in symbols {
        event!(Level::DEBUG, "r={}, c={}", row, col);
        let mut pos: HashSet<(Option<usize>, Option<usize>)> = HashSet::new();
        pos.insert((row.checked_sub(1), Some(*col)));
        pos.insert((Some(*row), col.checked_sub(1)));
        pos.insert((Some(*row), col.checked_add(1)));
        pos.insert((row.checked_add(1), Some(*col)));
        pos.insert((row.checked_sub(1), col.checked_sub(1)));
        pos.insert((row.checked_sub(1), col.checked_add(1)));
        pos.insert((row.checked_add(1), col.checked_add(1)));
        pos.insert((row.checked_add(1), col.checked_sub(1)));

        let adj_digits = pos.into_iter()
            .filter(|(r, c)| r.is_some() && c.is_some())
            .map(|(r, c)| (r.unwrap(), c.unwrap()))
            .filter(|(r, c)| {
                let is_valid = is_valid_pos((*r, *c), row_max, col_max);
                return is_valid && l[*r][*c].is_digit(10);
            }).collect::<Vec<(usize, usize)>>();
        event!(Level::DEBUG, "adj_digits={:?}", adj_digits);

        let mut digits: HashSet<u32> = HashSet::new();
        for (r, c) in adj_digits {
            let mut cur_col = c;
            let mut digit = "".to_owned();

            // Check left 
            while l[r][cur_col].is_digit(10) {
                // preped onto string
                digit.insert(0, l[r][cur_col]); 
                if cur_col == 0 { break; }
                cur_col -= 1;
            }
            
            // Check right
            cur_col = c+1;
            while l[r][cur_col].is_digit(10) {
                // append onto string
                digit.push(l[r][cur_col]); 
                if cur_col == col_max-1 { break; }
                cur_col += 1;
            }
            digits.insert(digit.parse::<u32>().unwrap());
        }
        event!(Level::DEBUG, "digits={:?}", digits);
        if digits.len() == 2 {
            event!(Level::INFO, "found valid gears {:?}", digits);
            res.push(digits.into_iter().product());
        }
    }


    return res;
}

#[instrument(skip(s))]
fn solve_2(s: &'static str) -> u32 {
    let l = parse_file(s);
    let symbols: Vec<(usize, usize)> = find_symbols(l.clone())
        .into_iter().filter(|(r, c)| l[*r][*c] == '*')
        .collect::<Vec<(usize, usize)>>();
    event!(Level::TRACE, "symbols={:?}", symbols);

    return find_adj_digits(l, &symbols).into_iter().sum();
}

#[instrument(skip(s))]
fn solve_1(s: &'static str) -> u32 {
    // Transfer file into 2D container.
    let l = parse_file(s);

    // To store all found part numbers
    let mut part_numbers: Vec<u32> = Vec::new();

    for (i, line) in l.iter().enumerate() {
        let mut num: String = "".to_owned();
        let mut start: Option<usize> = None;

        for (j, ch) in line.iter().enumerate() {
            let is_digit = ch.is_digit(10);

            if is_digit {
                num.push(*ch);
                if start.is_none() { start = Some(j) }
            } 

            if (is_digit && j == line.len()-1) || (!is_digit && !num.is_empty()) {
                // hit a non-digit, but number is contained
                event!(Level::INFO, "Found number {}", num);
    
                if search_adj(i, start.unwrap()..=j-1, l.clone()) {
                    event!(Level::INFO, "Number {} is VALID", num);
                    part_numbers.push(num.parse::<u32>().unwrap());
                }

                // Reset for next number
                start = None;
                num.clear();
            }
        }
    }  

    return part_numbers.iter().sum::<u32>();
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
