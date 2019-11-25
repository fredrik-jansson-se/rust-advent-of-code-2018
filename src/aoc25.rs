use super::helper::i32_val;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::opt;
use nom::multi::{many0, separated_list};
use nom::sequence::terminated;
use nom::IResult;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day25.txt").unwrap();
    let (_, vecs) = parse(&input).unwrap();
    // let vecs = vecs.iter().filter(|v| !v.is_empty()).collect();
    // println!("25:1 {}", run_1(&vecs));
    println!("{:?}", vecs);
}

fn parse_vector(i: &str) -> IResult<&str, Vec<i32>> {
    terminated(separated_list(tag(","), i32_val), opt(newline))(i)
}

fn parse(i: &str) -> IResult<&str, Vec<Vec<i32>>> {
    // separated_list(newline, parse_vector)(i)
    many0(parse_vector)(i)
}

fn m_distance(v1: &[i32], v2: &[i32]) -> i32 {
    v1.iter().zip(v2.iter()).map(|(a, b)| (a - b).abs()).sum()
}

fn in_constellation(v1: &[i32], constellation: &[Vec<i32>], distance: i32) -> bool {
    constellation
        .iter()
        .any(|v2| m_distance(v1, v2) <= distance)
}

fn run_1(input: &[Vec<i32>]) -> usize {
    let mut v = input.to_owned();
    let mut constellations = Vec::new();

    let mut current = Vec::new();

    // Add the first one to the current constellation
    current.push(v.pop().unwrap());

    while !v.is_empty() {
        loop {
            let (mut i, o): (Vec<Vec<i32>>, Vec<Vec<i32>>) = v
                .into_iter()
                .partition(|v1| in_constellation(v1, &current, 3));

            v = o;
            if i.is_empty() {
                break;
            }

            current.append(&mut i);
        }
        constellations.push(current);
        current = Vec::new();
        if v.len() > 0 {
            current.push(v.pop().unwrap());
        }
    }

    if !current.is_empty() {
        constellations.push(current);
    }

    // dbg! { &constellations};

    constellations.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc25_parse_vector() {
        assert_eq!(parse_vector("-1,2,2,0\n"), Ok(("", vec![-1, 2, 2, 0])));
        assert_eq!(parse_vector("0,0,2,-2\n"), Ok(("", vec![0, 0, 2, -2])));
        assert_eq!(parse_vector("0,0,2,-2"), Ok(("", vec![0, 0, 2, -2])));
        assert_eq!(
            parse_vector("0,0,2,-2\n-1,2,2,0"),
            Ok(("-1,2,2,0", vec![0, 0, 2, -2]))
        );
        assert_eq!(
            dbg! {parse("0,0,2,-2\n-1,2,2,0\n")},
            Ok(("", vec![vec![0, 0, 2, -2], vec![-1, 2, 2, 0]]))
        );
    }

    #[test]
    fn aoc25_distance() {
        assert_eq!(m_distance(&[0, 0, 0, 0], &[3, 0, 0, 0]), 3);
        assert_eq!(m_distance(&[3, 0, 0, 0], &[0, 0, 0, 0]), 3);
    }

    #[test]
    fn aoc25_run_1() {
        let (_, input) = parse(
            "0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0",
        )
        .unwrap();

        assert_eq!(run_1(&input), 2);

        let (_, input) = parse(
            "-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0",
        )
        .unwrap();

        assert_eq!(run_1(&input), 4);

        let (_, input) = parse(
            "1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2",
        )
        .unwrap();

        assert_eq!(run_1(&input), 3);

        let (_, input) = parse(
            "1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2",
        )
        .unwrap();

        assert_eq!(run_1(&input), 8);
    }
}
