use std::cmp::Ordering;
use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Up,
    Down,
    Right,
}

impl Direction {
    fn left(&self) -> Self {
        match self {
            Self::Left => Self::Down,
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    fn right(&self) -> Self {
        match self {
            Self::Left => Self::Up,
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Right => Self::Down,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Position {
    Horizontal,
    Vertical,
    NE,
    SE,
    SW,
    NW,
    Intersection,
}

#[derive(Debug, PartialEq, Eq)]
struct Cart {
    dir: Direction,
    intersect_dir: usize,
    x: isize,
    y: isize,
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.y == other.y {
            self.x.partial_cmp(&other.x)
        } else {
            self.y.partial_cmp(&other.y)
        }
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y == other.y {
            self.x.cmp(&other.x)
        } else {
            self.y.cmp(&other.y)
        }
    }
}

impl Cart {
    fn next_pos(&self, dir: &Direction) -> (isize, isize) {
        match dir {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        }
    }

    fn mv(&mut self, map: &Map) {
        match map[self.y as usize][self.x as usize] {
            Some(Position::Intersection) => match self.intersect_dir {
                0 => {
                    let (x, y) = self.next_pos(&self.dir.left());
                    self.x = x;
                    self.y = y;
                    self.intersect_dir = 1;
                    self.dir = self.dir.left();
                }
                1 => {
                    let (x, y) = self.next_pos(&self.dir);
                    self.x = x;
                    self.y = y;
                    self.intersect_dir = 2;
                }
                2 => {
                    let (x, y) = self.next_pos(&self.dir.right());
                    self.x = x;
                    self.y = y;
                    self.intersect_dir = 0;
                    self.dir = self.dir.right();
                }
                _ => unreachable!(),
            },
            Some(Position::Horizontal) if self.dir == Direction::Right => self.x += 1,
            Some(Position::Horizontal) => self.x -= 1,
            Some(Position::Vertical) if self.dir == Direction::Down => self.y += 1,
            Some(Position::Vertical) => self.y -= 1,
            Some(Position::NW) if self.dir == Direction::Up => {
                self.x += 1;
                self.dir = Direction::Right;
            }
            Some(Position::NW) => {
                self.y += 1;
                self.dir = Direction::Down;
            }
            Some(Position::NE) if self.dir == Direction::Up => {
                self.x -= 1;
                self.dir = Direction::Left;
            }
            Some(Position::NE) => {
                self.y += 1;
                self.dir = Direction::Down;
            }
            Some(Position::SW) if self.dir == Direction::Down => {
                self.x += 1;
                self.dir = Direction::Right;
            }
            Some(Position::SW) => {
                self.y -= 1;
                self.dir = Direction::Up;
            }
            Some(Position::SE) if self.dir == Direction::Down => {
                self.x -= 1;
                self.dir = Direction::Left;
            }
            Some(Position::SE) => {
                self.y -= 1;
                self.dir = Direction::Up;
            }

            None => {
                dbg! { &self};
                dbg! { &map[self.y as usize - 1][self.x as usize ]};
                unreachable!();
            }
        }
    }
}

type Map = Vec<Vec<Option<Position>>>;

fn is_valid((x, y): (isize, isize), map: &Map) -> bool {
    x >= 0
        && y >= 0
        && (y as usize) < map.len()
        && (x as usize) < map[y as usize].len()
        && map[y as usize][x as usize].is_some()
}

fn parse_map(input: &str) -> (Map, Vec<Cart>) {
    let mut max_width = 0;
    let mut map = Map::new();
    let mut carts = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        let mut last = None;
        for (x, c) in line.chars().enumerate() {
            match c {
                '<' => {
                    carts.push(Cart {
                        dir: Direction::Left,
                        intersect_dir: 0,
                        x: x as isize,
                        y: y as isize,
                    });
                    row.push(Some(Position::Horizontal));
                }
                '>' => {
                    carts.push(Cart {
                        dir: Direction::Right,
                        intersect_dir: 0,
                        x: x as isize,
                        y: y as isize,
                    });
                    row.push(Some(Position::Horizontal));
                }
                '^' => {
                    carts.push(Cart {
                        dir: Direction::Up,
                        intersect_dir: 0,
                        x: x as isize,
                        y: y as isize,
                    });
                    row.push(Some(Position::Vertical));
                }
                'v' => {
                    carts.push(Cart {
                        dir: Direction::Down,
                        intersect_dir: 0,
                        x: x as isize,
                        y: y as isize,
                    });
                    row.push(Some(Position::Vertical));
                }
                '-' => row.push(Some(Position::Horizontal)),
                '|' => row.push(Some(Position::Vertical)),
                '/' if last.is_none()
                    || Some(Position::Vertical) == last
                    || Some(Position::NE) == last =>
                {
                    row.push(Some(Position::NW))
                }
                '/' => row.push(Some(Position::SE)),
                '\\' if last.is_none()
                    || Some(Position::Vertical) == last
                    || Some(Position::SE) == last =>
                {
                    row.push(Some(Position::SW))
                }
                '\\' => row.push(Some(Position::NE)),
                '+' => row.push(Some(Position::Intersection)),
                ' ' => row.push(None),

                any => {
                    dbg! {any};
                    unreachable!()
                }
            }
            last = row[row.len() - 1].clone();
        }

        max_width = usize::max(max_width, row.len());

        map.push(row);
    }

    // Make all rows the same width
    for row in map.iter_mut() {
        while row.len() < max_width {
            row.push(None)
        }
    }

    (map, carts)
}

fn printmap(m: &Map, carts: &[Cart]) {
    for (y, row) in m.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            let cart = carts
                .iter()
                .find(|c| (x as isize, y as isize) == (c.x, c.y));
            if let Some(cart) = cart {
                match cart.dir {
                    Direction::Right => print!(">"),
                    Direction::Left => print!("<"),
                    Direction::Up => print!("^"),
                    Direction::Down => print!("v"),
                }
            } else {
                match col {
                    None => print!(" "),
                    Some(Position::Horizontal) => print!("-"),
                    Some(Position::Vertical) => print!("|"),
                    Some(Position::NE) => print!(r#"\"#),
                    Some(Position::NW) => print!("/"),
                    Some(Position::SW) => print!(r#"\"#),
                    Some(Position::SE) => print!("/"),
                    Some(Position::Intersection) => print!("+"),
                }
            }
        }
        println!();
    }
    println!();
}

pub fn run() {
    let input = fs::read_to_string("day13.txt").unwrap();
    let (x, y) = run_1(&input);
    println!("13:1 {},{}", x, y);
    let (x, y) = run_2(&input);
    println!("13:2 {},{}", x, y);
}

fn run_1(input: &str) -> (isize, isize) {
    let (map, mut carts) = parse_map(input);
    loop {
        // First sort the carts per their position
        carts.sort();

        for i in 0..carts.len() {
            carts[i].mv(&map);
            for j in 0..carts.len() {
                if i != j && (carts[i].x, carts[i].y) == (carts[j].x, carts[j].y) {
                    // printmap(&map, &carts);
                    return (carts[i].x, carts[i].y);
                }
            }
        }
    }
}

fn run_2(input: &str) -> (isize, isize) {
    let (map, mut carts) = parse_map(input);
    loop {
        // First sort the carts per their position
        carts.sort();

        if carts.len() == 1 {
            break;
        }

        let mut to_remove = Vec::new();

        for i in 0..carts.len() {
            carts[i].mv(&map);
            for j in 0..carts.len() {
                if i != j && (carts[i].x, carts[i].y) == (carts[j].x, carts[j].y) {
                    to_remove.push(i);
                    to_remove.push(j);
                }
            }
        }
        to_remove.sort();
        to_remove.reverse();
        for i in to_remove {
            carts.remove(i);
        }
    }
    (carts[0].x, carts[0].y)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn map_1() -> &'static str {
        r#"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/"#
    }

    #[test]
    fn aoc13_advance() {
        let (m, mut c) = parse_map(map_1());
        // dbg! {&m};
        c.iter_mut().for_each(|c| c.mv(&m));
        assert_eq!((3, 0), (c[0].x, c[0].y));
        assert_eq!(Direction::Right, c[0].dir);
        assert_eq!((9, 4), (c[1].x, c[1].y));
        assert_eq!(Direction::Down, c[1].dir);

        c.iter_mut().for_each(|c| c.mv(&m));
        c.iter_mut().for_each(|c| c.mv(&m));
        printmap(&m, &c);
        assert_eq!((4, 1), (c[0].x, c[0].y));
        assert_eq!(Direction::Down, c[0].dir);
        assert_eq!((11, 4), (c[1].x, c[1].y));
        assert_eq!(Direction::Right, c[1].dir);
    }

    #[test]
    fn aoc13_1() {
        let (m, c) = parse_map(map_1());
        printmap(&m, &c);
        assert_eq!(2, c.len());
        assert_eq!((7, 3), run_1(map_1()));
    }

    fn map_2() -> &'static str {
        r#"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"#
    }

    #[test]
    fn aoc13_2() {
        let (m, c) = parse_map(map_2());
        printmap(&m, &c);
        assert_eq!(9, c.len());
        assert_eq!((6, 4), run_2(map_2()));
    }
}
