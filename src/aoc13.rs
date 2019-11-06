// use std::fs;

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Up,
    Down,
    Right,
}

#[derive(Debug)]
struct Cart {
    dir: Direction,
    x: isize,
    y: isize,
}

impl Cart {
    fn next_pos(&self) -> (isize, isize) {
        match self.dir {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        }
    }
}

type Map = Vec<Vec<bool>>;

fn is_valid((x, y): (isize, isize), map: &Map) -> bool {
    x >= 0
        && y >= 0
        && (x as usize) < map[y as usize].len()
        && (y as usize) < map.len()
        && map[y as usize][x as usize]
}

fn parse_map(input: &str) -> (Map, Vec<Cart>) {
    let mut max_width = 0;
    let mut map = Map::new();
    let mut carts = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                '<' => {
                    carts.push(Cart {
                        dir: Direction::Left,
                        x: x as isize,
                        y: y as isize,
                    });
                    row.push(true);
                }
                '>' => {
                    carts.push(Cart {
                        dir: Direction::Right,
                        x: x as isize,
                        y: y as isize,
                    });
                    row.push(true);
                }
                '^' => {
                    carts.push(Cart {
                        dir: Direction::Up,
                        x: x as isize,
                        y: y as isize,
                    });
                    row.push(true);
                }
                'v' => {
                    carts.push(Cart {
                        dir: Direction::Down,
                        x: x as isize,
                        y: y as isize,
                    });
                    row.push(true);
                }
                ' ' => {
                    row.push(false);
                }
                _ => {
                    row.push(true);
                }
            }
        }

        max_width = usize::max(max_width, row.len());

        map.push(row);
    }

    // Make all rows the same width
    for row in map.iter_mut() {
        while row.len() < max_width {
            row.push(false);
        }
    }

    (map, carts)
}

fn printmap(m: &Map) {
    for row in m {
        for col in row {
            if *col {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

pub fn run() {}

#[cfg(test)]
mod tests {
    use super::*;
    fn map() -> &'static str {
        r#"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/"#
    }

    #[test]
    fn aoc13_1() {
        let (m, c) = parse_map(map());
        printmap(&m);
        assert_eq!(2, c.len());
    }
}
