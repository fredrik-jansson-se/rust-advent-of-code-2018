use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day3.txt").unwrap();

    println!("3:1 - {}", run_1(&input));
    println!("3:2 - {}", run_2(&input));
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Claim {
    fn new(id: u32, x: u32, y: u32, w: u32, h: u32) -> Claim {
        Claim {
            id: id,
            x: x,
            y: y,
            width: w,
            height: h,
        }
    }
}

fn s2i(s: &str) -> u32 {
    u32::from_str_radix(s, 10).unwrap()
}

fn parse(row: &str) -> Claim {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"#(\d+)\s+@\s+(\d+),(\d+):\s+(\d+)x(\d+)"#).unwrap();
    }

    let c = RE.captures(row).unwrap();
    Claim::new(s2i(&c[1]), s2i(&c[2]), s2i(&c[3]), s2i(&c[4]), s2i(&c[5]))
}

fn run_1(input: &str) -> u32 {
    let components: Vec<Claim> = input.lines().map(parse).collect();
    let mut clms = HashMap::new();

    for c in components {
        for x in c.x..(c.x + c.width) {
            for y in c.y..(c.y + c.height) {
                let counter = clms.entry((x, y)).or_insert(0);
                *counter += 1;
            }
        }
    }
    let mut cnt = 0;
    for (_, v) in clms {
        if v > 1 {
            cnt += 1;
        }
    }
    cnt
}

fn run_2(input: &str) -> u32 {
    let components: Vec<Claim> = input.lines().map(parse).collect();
    let mut clms = HashMap::new();

    for c in components.iter() {
        for x in c.x..(c.x + c.width) {
            for y in c.y..(c.y + c.height) {
                let counter = clms.entry((x, y)).or_insert(0);
                *counter += 1;
            }
        }
    }

    for c in components.iter() {
        let mut found = false;
        for x in c.x..(c.x + c.width) {
            for y in c.y..(c.y + c.height) {
                if *clms.get(&(x, y)).unwrap() > 1 {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        if !found {
            return c.id;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc3_parse() {
        assert_eq!(
            Claim::new(1, 662, 777, 18, 27),
            parse("#1 @ 662,777: 18x27")
        );
    }

    #[test]
    fn aoc3_run_1() {
        assert_eq!(4, run_1("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"));
    }

    #[test]
    fn aoc3_run_2() {
        assert_eq!(3, run_2("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"));
    }
}
