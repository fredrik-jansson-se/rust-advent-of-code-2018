// use std::fs;

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Up,
    Down,
    Right,
}

enum Point {
    None,
    Cart(Direction),
}

pub fn run() {}

#[cfg(test)]
mod tests {
    // use super::*;
    fn map() -> &'static str {
        r#"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/"#
    }

}
