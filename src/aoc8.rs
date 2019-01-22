use std::fs;

pub fn run() {
    let input = fs::read_to_string("day8.txt").unwrap();
    println!("day8-1: {}", run_1(&input[0..(input.len() - 1)]));
    println!("day8-2: {}", run_2(&input[0..(input.len() - 1)]));
}

#[derive(Debug)]
struct Node {
    metadata: Vec<u32>,
    children: Vec<Node>,
}

fn to_vec(input: &str) -> Vec<u32> {
    input
        .split(" ")
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn parse_child(nodes: &[u32], mut idx: usize) -> (Node, usize) {
    let no_children = nodes[idx];
    idx += 1;
    let no_metdata = nodes[idx];
    idx += 1;

    let mut children = Vec::new();
    for _ in 0..no_children {
        let (c, new_idx) = parse_child(nodes, idx);
        children.push(c);
        idx = new_idx;
    }

    let mut metadata = Vec::new();
    for _ in 0..no_metdata {
        metadata.push(nodes[idx]);
        idx += 1;
    }

    (
        Node {
            metadata: metadata,
            children: children,
        },
        idx,
    )
}

fn sum_metadata(child: &Node) -> u32 {
    let msum: u32 = child.metadata.iter().sum();
    msum + child
        .children
        .iter()
        .fold(0, |sum, c| sum + sum_metadata(c))
}

fn run_1(input: &str) -> u32 {
    let node_input = to_vec(input);

    let (child, _) = parse_child(&node_input, 0);

    sum_metadata(&child)
}

fn value(node: &Node) -> u32 {
    if node.children.len() == 0 {
        node.metadata.iter().sum()
    } else {
        let mut sum = 0;
        for md in node.metadata.iter() {
            let idx = *md as usize - 1;
            if idx < node.children.len() {
                sum += value(&node.children[idx]);
            }
        }
        sum
    }
}

fn run_2(input: &str) -> u32 {
    let node_input = to_vec(input);

    let (child, _) = parse_child(&node_input, 0);

    value(&child)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc8_to_vec() {
        assert_eq!(to_vec("1 2 3 4"), [1, 2, 3, 4]);
        assert_eq!(to_vec("10 20 30 40"), [10, 20, 30, 40]);
    }

    #[test]
    fn aoc8_run_1() {
        assert_eq!(run_1("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), 138);
    }

    #[test]
    fn aoc8_run_2() {
        assert_eq!(run_2("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), 66);
    }
}
