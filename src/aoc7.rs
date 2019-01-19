use regex::Regex;
use std::fs;
use std::str;

use lazy_static;


pub fn run() {
    let input = fs::read_to_string("day7.txt").unwrap();
    println!("day7-1: {}", run_1(&input));
}

fn parse_line(line: &str) -> (char, char) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"Step\s(\w).*step\s(\w).*\."#).unwrap();
    }
    ('a', 'b')
}

pub fn run_1(input: &str) -> String {
    dbg!(input);
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc7_run_1() {
        let input = r#"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."#;
        assert_eq!("CABDFE", run_1(input));
    }

    #[test]
    fn aoc7_run_2() {
        // assert_eq!(4, run_2("dabAcCaCBAcCcaDA".to_string()));
    }
}
