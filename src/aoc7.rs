use regex::Regex;
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::str;

use lazy_static;

pub fn run() {
    let input = fs::read_to_string("day7.txt").unwrap();
    println!("day7-1: {}", run_1(&input));
    // 2862
    println!("day7-2: {}", run_2(&input, 5, 60));
}

fn parse_line(line: &str) -> (String, String) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"Step\s(\w).*step\s(\w).*\."#).unwrap();
    }
    let c = RE.captures(line).unwrap();
    let c1 = c[1].to_string();
    let c2 = c[2].to_string();
    (c1, c2)
}

pub fn run_1(input: &str) -> String {
    let components: Vec<(String, String)> = input.lines().map(parse_line).collect();
    let mut lookup: HashMap<String, BTreeSet<String>> = HashMap::new();
    let mut open = BTreeSet::new();
    let mut closed = BTreeSet::new();
    for (c1, c2) in components {
        if !closed.contains(&c1) {
            open.insert(c1.clone());
        }
        open.remove(&c2);
        closed.insert(c2.clone());
        if let Some(s) = lookup.get_mut(&c1) {
            s.insert(c2);
        } else {
            let mut s = BTreeSet::new();
            s.insert(c2);
            lookup.insert(c1, s);
        }
    }

    let mut res: Vec<String> = Vec::new();

    while !open.is_empty() {
        let o = open.iter().next().unwrap().clone();
        open.remove(&o);
        if let Some(children) = lookup.get(&o) {
            for v in children.iter() {
                open.insert(v.clone());
            }
        }
        res = res.into_iter().filter(|c| *c != o).collect();
        res.push(o);
    }

    res.join("")
}

#[derive(Debug, Clone)]
struct WorkItem {
    name: String,
    seconds_left: usize,
}

impl WorkItem {
    fn new(name: &str, base_cost: usize) -> Self {
        WorkItem {
            name: name.to_owned(),
            seconds_left: base_cost + step_cost(name.chars().next().unwrap()),
        }
    }

    fn tick(&mut self) -> bool {
        // dbg! { &self };
        self.seconds_left-= 1;
        self.seconds_left == 0
    }
}

pub fn run_2(input: &str, num_workers: usize, base_cost: usize) -> u32 {
    let components: Vec<(String, String)> = input.lines().map(parse_line).collect();
    let mut lookup: HashMap<String, BTreeSet<String>> = HashMap::new();
    let mut open = BTreeSet::new();
    let mut closed = BTreeSet::new();
    for (c1, c2) in components {
        if !closed.contains(&c1) {
            open.insert(c1.clone());
        }
        open.remove(&c2);
        closed.insert(c2.clone());
        if let Some(s) = lookup.get_mut(&c1) {
            s.insert(c2);
        } else {
            let mut s = BTreeSet::new();
            s.insert(c2);
            lookup.insert(c1, s);
        }
    }

    // dbg!{ &open };

    let mut workers = vec![None; num_workers];

    {
        let o = open.iter().next().unwrap().clone();
        open.remove(&o);
        workers[0]=Some(WorkItem::new(&o, base_cost));
    }

    let mut sum = 0;

    loop {
        if workers.iter().all(|i| i.is_none()) {
            break;
        }
        sum += 1;

        for w in workers.iter_mut() {
            match w {
                Some(wi) => if wi.tick() {
                    // This is done, get the children
                    if let Some(children) = lookup.get(&wi.name) {
                        for v in children.iter() {
                            open.insert(v.clone());
                        }
                    }
                    if open.len() > 0 {
                        let o = open.iter().next().unwrap().clone();
                        open.remove(&o);
                        *w=Some(WorkItem::new(&o, base_cost));
                    }
                    else {
                        *w = None;
                    }
                }
                None => {
                    if open.len() > 0 {
                        let o = open.iter().next().unwrap().clone();
                        open.remove(&o);
                        *w=Some(WorkItem::new(&o, base_cost));
                    }
                }
            }
        }
    }

    sum
}

fn step_cost(c: char) -> usize {
    let lookup = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    lookup.find(c).unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc7_parse_line() {
        assert_eq!(
            ("C".to_string(), "F".to_string()),
            parse_line("Step C must be finished before step F can begin.")
        );
    }

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
        assert_eq!(step_cost('A'), 1);
        assert_eq!(step_cost('B'), 2);
        let input = r#"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."#;
        assert_eq!(15, run_2(input, 2, 0));
    }
}
