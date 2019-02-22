use std::fs;
use nom::types::CompleteStr;
use nom::*;
use std::ops::AddAssign;
use super::helper::*;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn run() {
    let input = fs::read_to_string("day10.txt").unwrap();
    println!("10:1 && 2: {}", run_1(&input));
}

#[derive(Clone, Debug, PartialEq)]
struct Vector {
    x: i32,
    y: i32,
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Point {
    pos: Vector,
    vel: Vector,
}

impl Point {
    fn advance(&mut self) -> &mut Point {
        self.pos += self.vel.clone();
        self
    }
}

// < 7,  0>
named!(vector<CompleteStr, Vector>, do_parse!(
        tag!("<") >>
        opt!(space) >>
        x: i32_val >>
        tag!(",") >>
        opt!(space) >>
        y: i32_val >>
        tag!(">") >>
        ( Vector {x, y} )
        ));

// position=< 7,  0> velocity=<-1,  0>
named!(point<CompleteStr, Point>, do_parse!(
        opt!(space) >>
        tag!("position=") >>
        pos: vector >>
        opt!(space) >>
        tag!("velocity=") >>
        vel: vector >>
        opt!(tag!("\n")) >>
        ( Point { pos, vel })
        ));

named!(points<CompleteStr, Vec<Point>>, many1!(point));

fn bounds(points: &[Point]) -> (Vector, Vector) {
    let mut x_min = i32::max_value();
    let mut y_min = i32::max_value();
    let mut x_max = i32::min_value();
    let mut y_max = i32::min_value();

    for p in points.iter() {
        x_min = i32::min(x_min, p.pos.x);
        y_min = i32::min(y_min, p.pos.y);
        x_max = i32::max(x_max, p.pos.x);
        y_max = i32::max(y_max, p.pos.y);
    }


    ( Vector { x: x_min, y: y_min},
      Vector { x: x_max, y: y_max })
}

fn print_field((min, max): &(Vector, Vector), points: &[Point]) {
    let spoints : HashSet<(i32, i32)> = HashSet::from_iter(
        points.iter().map(|p| (p.pos.x, p.pos.y)));

    for y in min.y..(max.y+1) {
        for x in min.x..(max.x+1) {
            if spoints.contains(&(x, y)) {
                print!("*");
            }
            else {
                print!(".");
            }
        }
        println!("");
    }
}

fn bounds_area((min, max): &(Vector, Vector)) ->i64 {
    let dx = (max.x - min.x).abs() as i64;
    let dy = (max.y - min.y).abs() as i64;
    dx * dy
}

fn run_1(input: &str) -> u32 {
    let (_, mut pts) = points(CompleteStr(input)).unwrap();

    let mut last_area = i64::max_value();
    let mut last_pts = pts.clone();

    for i in 0.. {
        let bounds = bounds(&pts);
        let bounds_area = bounds_area(&bounds);
        if last_area < bounds_area {
            print_field(&bounds, &last_pts);
            return i-1;
        }
        last_area = bounds_area;

        last_pts = pts.clone();
        for p in pts.iter_mut() {
            p.advance();
        }
    }
    pts.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_input() -> &'static str {
    r#"position=< 9,  1> velocity=< 0,  2>
    position=< 7,  0> velocity=<-1,  0>
    position=< 3, -2> velocity=<-1,  1>
    position=< 6, 10> velocity=<-2, -1>
    position=< 2, -4> velocity=< 2,  2>
    position=<-6, 10> velocity=< 2, -2>
    position=< 1,  8> velocity=< 1, -1>
    position=< 1,  7> velocity=< 1,  0>
    position=<-3, 11> velocity=< 1, -2>
    position=< 7,  6> velocity=<-1, -1>
    position=<-2,  3> velocity=< 1,  0>
    position=<-4,  3> velocity=< 2,  0>
    position=<10, -3> velocity=<-1,  1>
    position=< 5, 11> velocity=< 1, -2>
    position=< 4,  7> velocity=< 0, -1>
    position=< 8, -2> velocity=< 0,  1>
    position=<15,  0> velocity=<-2,  0>
    position=< 1,  6> velocity=< 1,  0>
    position=< 8,  9> velocity=< 0, -1>
    position=< 3,  3> velocity=<-1,  1>
    position=< 0,  5> velocity=< 0, -1>
    position=<-2,  2> velocity=< 2,  0>
    position=< 5, -2> velocity=< 1,  2>
    position=< 1,  4> velocity=< 2,  1>
    position=<-2,  7> velocity=< 2, -2>
    position=< 3,  6> velocity=<-1, -1>
    position=< 5,  0> velocity=< 1,  0>
    position=<-6,  0> velocity=< 2,  0>
    position=< 5,  9> velocity=< 1, -2>
    position=<14,  7> velocity=<-2,  0>
    position=<-3,  6> velocity=< 2, -1>"#
    }

    #[test]
    fn aoc10_parse() {
        assert_eq!(vector(CompleteStr("< 8, -9>")), Ok((CompleteStr(""), Vector {x: 8, y: -9})));
        assert_eq!(vector(CompleteStr("<-8, 9>")), Ok((CompleteStr(""), Vector {x: -8, y: 9})));
        assert_eq!(point(CompleteStr("position=<-3,  6> velocity=< 2, -1>\n")), Ok((CompleteStr(""), Point { pos: Vector {x: -3, y: 6}, vel: Vector { x: 2, y: -1}})));
        assert_eq!(point(CompleteStr("position=<15,  0> velocity=<-2,  0>")), Ok((CompleteStr(""), Point { pos: Vector {x: 15, y: 0}, vel: Vector { x: -2, y: 0}})));
        let (rest, pts) = points(CompleteStr(test_input())).unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(pts.len(), 31);
    }

    #[test]
    fn aoc10_advance() {
        let mut p = Point {
            pos: Vector { x: 3, y: 9},
            vel: Vector { x: 1, y: -2}
        };

        assert_eq!(p.advance().advance().advance().pos, Vector { x: 6, y: 3});
    }

    #[test]
    fn aoc10_run_1() {
        assert_eq!(run_1(test_input()), 3);
    }
}
