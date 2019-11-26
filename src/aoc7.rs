use regex::Regex;
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::str;

use lazy_static;

pub fn run() {
    let input = fs::read_to_string("day7.txt").unwrap();
    println!("day7-1: {}", run_1(&input));
    // 897 // too high
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
        let s = lookup.entry(c1).or_insert(BTreeSet::new());
        s.insert(c2);
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
        self.seconds_left -= 1;
        self.seconds_left == 0
    }
}

fn print_workers(sec: u32, workers: &[Option<WorkItem>]) {
    print!("{:03} - ", sec);
    for w in workers {
        match w {
            None => print!(". "),
            Some(wi) => print!("{} ", &wi.name),
        }
    }
    println!("");
}

pub fn run_2(input: &str, num_workers: usize, base_cost: usize) -> u32 {
    let components: Vec<(String, String)> = input.lines().map(parse_line).collect();
    let mut lookup: HashMap<String, BTreeSet<String>> = HashMap::new();
    let mut required: HashMap<String, BTreeSet<String>> = HashMap::new();
    let mut open = BTreeSet::new();
    let mut closed = BTreeSet::new();
    for (c1, c2) in components {
        if !closed.contains(&c1) {
            open.insert(c1.clone());
        }
        open.remove(&c2);
        closed.insert(c2.clone());
        let s = lookup.entry(c1.clone()).or_insert(BTreeSet::new());
        s.insert(c2.clone());
        let s = required.entry(c2).or_insert(BTreeSet::new());
        s.insert(c1);
    }

    let mut workers = vec![None; num_workers];

    for w in workers.iter_mut() {
        if open.is_empty() {
            break;
        }
        if w.is_none() {
            let o = open.iter().next().unwrap().clone();
            open.remove(&o);
            *w = Some(WorkItem::new(&o, base_cost));
        }
    }

    let mut sum = 0;
    let mut sec = 0;

    loop {
        if workers.iter().all(|i| i.is_none()) {
            break;
        }
        sum += 1;
        sec += 1;

        print_workers(sec, &workers);

        for w in workers.iter_mut() {
            if let Some(wi) = w {
                if wi.tick() {
                    // This is done, get the children
                    if let Some(children) = lookup.get(&wi.name) {
                        for v in children.iter() {
                            // Remove this work item from the requirements for the child
                            if let Some(r) = required.get_mut(v) {
                                r.remove(&wi.name);
                                // If we don't have any more requirements, add it to the open list
                                if r.is_empty() {
                                    required.remove(v);
                                    open.insert(v.clone());
                                }
                            }
                        }
                    }
                    *w = None;
                }
            }
        }

        // dbg! { &open};
        // Assign any open item to a worker ( if any free worker)
        for w in workers.iter_mut() {
            if open.is_empty() {
                break;
            }
            if w.is_none() {
                let o = open.iter().next().unwrap().clone();
                open.remove(&o);
                *w = Some(WorkItem::new(&o, base_cost));
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
    fn aoc7_run_step_cost() {
        assert_eq!(step_cost('A'), 1);
        assert_eq!(step_cost('B'), 2);
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
