use std::fs;

pub fn run() {
    let input = fs::read_to_string("day15.txt").unwrap();
    println!("15 {}", input.len());
}


#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn aoc15_parse() {
        // let map = r#"#######
// #.G...#
// #...EG#
// #.#.#G#
// #..G#E#
// #.....#
// #######  "#;
    }
}
