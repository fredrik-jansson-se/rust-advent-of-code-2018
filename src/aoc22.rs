pub fn run() {
    println!("22:1 {}", run_1(11991, (6, 797)));
}

// #[derive(Debug, PartialEq, Clone)]
// enum Type {
//     //    Unknown,
//     Rocky,
//     Wet,
//     Narrow,
// }

// impl Type {
//     fn new(erosion_level: usize) -> Self {
//         match erosion_level % 3 {
//             0 => Type::Rocky,
//             1 => Type::Wet,
//             _ => Type::Narrow,
//         }
//     }

//     fn risk_level(&self) -> usize {
//         match self {
//             Type::Rocky => 0,
//             Type::Wet => 1,
//             Type::Narrow => 2,
//         }
//     }
// }

type Map = Vec<Vec<u64>>;

fn create_map(width: usize, height: usize) -> Map {
    let mut res = Vec::with_capacity(height);

    for _ in 0..height {
        res.push(vec![0; width]);
    }

    res
}

fn geologic_index(x: usize, y: usize, target: &(usize, usize), map: &Map) -> u64 {
    if x == 0 && y == 0 {
        0
    } else if x == target.0 && y == target.1 {
        0
    } else if x == 0 {
        y as u64 * 48271
    } else if y == 0 {
        x as u64 * 16807
    } else {
        // dbg!((x, y, map[y - 1][x], map[y][x - 1]));
        map[y - 1][x] * map[y][x - 1]
    }
}

fn erosion_level(x: usize, y: usize, target: &(usize, usize), map: &Map, depth: usize) -> u64 {
    (geologic_index(x, y, target, map) + depth as u64) % 20183
}

fn run_1(depth: usize, target: (usize, usize)) -> u64 {
    let width = target.0 + 1;
    let height = target.1 + 1;
    let mut map = create_map(width, height);

    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            let gl = erosion_level(x, y, &target, &map, depth);
            // dbg!((x, y, gl));
            map[y][x] = gl;
            sum += gl % 3;
        }
    }

    // dbg!(&map);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc22_geologic_index() {
        let map = create_map(11, 11);
        let target = (10, 10);
        assert_eq!(geologic_index(0, 0, &target, &map), 0);
        assert_eq!(geologic_index(10, 10, &target, &map), 0);
    }
    #[test]
    fn aoc22_test_1() {
        assert_eq!(run_1(510, (10, 10)), 114);
    }
}
