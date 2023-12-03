use core::fmt;
use std::str::FromStr;

const EX_1: &'static str = include_str!("../data/day2ex1.txt");
const PT_1: &'static str = include_str!("../data/day2p1.txt");

const MAX_RGB: RGB = RGB { 
    r: 12, g: 13, b: 14
};

#[cfg(test)]
mod tests {
    use crate::day2;

    #[test]
    fn example_1() {
        assert_eq!(day2::ex_1(), 8);
    }

    #[test]
    fn example_2() {
        assert_eq!(day2::ex_2(), 2286)
    }
}

#[derive(Debug, Clone)]
struct RGB_ERR;

impl fmt::Display for RGB_ERR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid parsing")
    }
}

#[derive(Debug, Clone, Copy)]
struct RGB {
    r: u32,
    g: u32,
    b: u32,
}

impl RGB {
    pub fn is_valid(&self) -> bool {
        return self.r <= MAX_RGB.r && self.g <= MAX_RGB.g && self.b <= MAX_RGB.b;
    }

    pub fn power(&self) -> u32 {
        return self.r * self.g * self.b;
    }
}

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(r={}, g={}, b={})", self.r, self.g, self.b)
    }
}

impl FromStr for RGB {
    type Err = RGB_ERR;

    fn from_str(line: &str) -> Result<Self, RGB_ERR> {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        // Split by n color, n color; n color
        //          |______________|  |_____|
  
        // Split by n color, n color; n color
        //          |_____|  |_____|  |_____|
        // println!("  Parsing \"{}\"", line);
        for s in line.split(';') {
            // println!("    splitting \"{}\" by ,", s);
            for color in s.split(',') {
                let mut iter = color.trim().splitn(2, ' ');
                // println!("      parsing color \"{:?}\"", iter.clone().collect::<Vec<_>>());

                let p_num: Result<u32, RGB_ERR> = iter.next()
                    .ok_or(RGB_ERR)
                    .and_then(|s| { 
                        s.trim().parse::<u32>().or(Err(RGB_ERR))
                    });

                if p_num.is_err() { return Err(RGB_ERR) }

                let num = p_num.unwrap();
                match iter.next().unwrap() {
                    "red"   => if num > r { r = num },
                    "green" => if num > g { g = num },
                    "blue"  => if num > b { b = num },
                    _ => (),
                }
            }
        }

        return Ok(RGB { r, g, b });
    }
}

fn get_rgb_from_line(line: &str) -> (u32, RGB) {
    // Split by Game <ID>: n color, n color; n color
    //          |_______|  |_______________________|
    let mut iter = line.splitn(2, ':');

    let game_id = iter.next().and_then(|s|  {
        let mut g_iter = s.splitn(2, ' ');

        g_iter.next();
        return Some(g_iter.next().unwrap().parse::<u32>().unwrap());
    });

    let rgb = iter.next().unwrap().parse::<RGB>().unwrap();
    return (game_id.unwrap(), rgb);
}


fn solve_1(s:&'static str) -> Result<u32, RGB_ERR> {
    return s.lines().map(|line: &str| -> Result<u32, RGB_ERR> {
        let (game_id, rgb) = get_rgb_from_line(line);

        if rgb.is_valid() { 
            // println!("  rgb {} is valid\n", rgb);
            return Ok(game_id) 
        } 
        // println!("  rgb {} is NOT valid\n", rgb);
        return Ok(0);
    }).reduce(|ra, rb| {
        if ra.is_err() || rb.is_err() { return Err(RGB_ERR) }
        return Ok(ra.unwrap() + rb.unwrap())
    }).unwrap_or(Err(RGB_ERR));
}

fn solve_2(s:&'static str) -> u32 {
    return s.lines().map(|line: &str| -> u32 {
        let (_, rgb) = get_rgb_from_line(line);

        // println!("{} has power {}", rgb, rgb.power());
        return rgb.power();
    }).sum::<u32>();
}


#[allow(dead_code)]
pub fn ex_1() -> u32 {
    return solve_1(EX_1).unwrap();
}

#[allow(dead_code)]
pub fn ex_2() -> u32 {
    return solve_2(EX_1);
}

pub fn pt_1() -> u32 {
    return solve_1(PT_1).unwrap();
}

pub fn pt_2() -> u32 {
    return solve_2(PT_1);
}
