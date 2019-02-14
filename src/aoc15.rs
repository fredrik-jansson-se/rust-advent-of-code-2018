use std::fs;

pub fn run() {
    let input = fs::read_to_string("day3.txt").unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc15_parse() {
        let map = r#"#######   
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#   
#######  "#;
    }
}
