use std::fs;
use nom::types::CompleteStr;
use nom::*;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use super::helper::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

named!(parse_coord<CompleteStr, Coord>, do_parse!(
        opt!(space) >>
        x: i32_val >>
        tag!(",") >>
        space >>
        y: i32_val >>
        opt!(tag!("\n")) >>
        (Coord { x, y} )
        ));

named!(parse<CompleteStr, Vec<Coord>>, many1!(parse_coord));

pub fn run() {
    let input = fs::read_to_string("day6.txt").unwrap();
    println!("6:1: {}", run_1(&input));
    println!("6:2: {}", run_2(&input, 10000));
}

fn distance(a: &Coord, b: &Coord) -> u32 {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as u32
}

fn run_1(input: &str) -> u32 {
    let (_, coords) = parse(CompleteStr(&input)).unwrap();
    let (c_min, c_max) = bounding_box(&coords);
    let mut non_borders: HashSet<Coord> = HashSet::from_iter(coords.clone());
    let mut areas = HashMap::new();
    for c in coords.iter() {
        areas.insert(c, 0);
    }
    for y in c_min.y..(c_max.y + 1) {
        for x in c_min.x..(c_max.x + 1) {
            let on_bounds = x == c_min.x || x >= c_max.x || y == c_min.y || y >= c_max.y;
            let a = Coord {x, y};

            let mut min_dist = u32::max_value();
            let mut current_node = None;

            for b in coords.iter() {
                let dist = distance(&a, b);
                if dist < min_dist {
                    min_dist = dist;
                    current_node = Some(b);
                }
                else if dist == min_dist {
                    // If several coords are equidistant, don't count
                    current_node = None;
                }
            }
            match current_node {
                Some(n) => {
                    if on_bounds {
                        non_borders.remove(current_node.unwrap());
                    }
                    let old = areas.get(n).unwrap();
                    areas.insert(n, old + 1);
                }
                None => ()
            }
        }
    }
    let max = areas.iter().filter(|(c, _) | non_borders.contains(c)).max_by(|(_, a), (_, b)| a.cmp(b));
    *max.unwrap().1
}

fn run_2(input: &str, max_distance: u32) -> u32 {
    let (_, coords) = parse(CompleteStr(&input)).unwrap();
    let (c_min, c_max) = bounding_box(&coords);
    let mut region_area = 0;
    for y in c_min.y..(c_max.y + 1) {
        for x in c_min.x..(c_max.x + 1) {
            let a = Coord {x, y};

            let total_distance = coords.iter().fold(0, |sum, b| sum + distance(&a, b));

            if total_distance < max_distance {
                region_area += 1;
            }
        }
    }
    region_area
}

fn bounding_box(coords: &[Coord]) -> (Coord, Coord) {
    let mut x_min=i32::max_value();
    let mut x_max = 0;
    let mut y_min=i32::max_value();
    let mut y_max = 0;
    for c in coords.iter() {
        x_min = std::cmp::min(x_min, c.x);
        x_max = std::cmp::max(x_max, c.x);
        y_min = std::cmp::min(y_min, c.y);
        y_max = std::cmp::max(y_max, c.y);
    }
    (
        Coord {x: x_min, y: y_min},
        Coord {x: x_max, y: y_max}
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc6_parse() {
        assert_eq!(parse_coord(CompleteStr("90, 110\n")), Ok((CompleteStr(""), Coord { x:90, y:110 })));
        assert_eq!(parse(CompleteStr("1, 2\n3, 4\n")), Ok((CompleteStr(""), vec![Coord { x:1, y: 2}, Coord { x:3, y:4} ])));
    }

    #[test]
    fn aoc6_run_1() {
       let input = r#"1, 1
       1, 6
       8, 3
       3, 4
       5, 5
       8, 9"#;
       assert_eq!(run_1(input), 17);
    }

    #[test]
    fn aoc6_run_2() {
       let input = r#"1, 1
       1, 6
       8, 3
       3, 4
       5, 5
       8, 9"#;
       assert_eq!(run_2(input, 32), 16);
    }

}
