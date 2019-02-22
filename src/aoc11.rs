use rayon::prelude::*;

pub fn run() {
    println!("11:1: {:?}", run_1(2866, 3));
    println!("11:2: {:?}", run_2(2866));
}

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn power_level(serial: i32, p: &Point) -> i64 {
    let rack_id = p.x as i64 + 10;
    let mut power_level = rack_id * p.y as i64;

    power_level += serial as i64;
    power_level *= rack_id;
    power_level /= 100;
    power_level %= 10;

    power_level - 5
}

fn create_power_map(serial: i32) -> Vec<Vec<i64>> {
    let mut power = vec![vec![0;300]; 300];
    for y in 1..301 {
        for x in 1..301 {
            power[y-1][x-1] = power_level(serial, &Point { x: x as i32, y: y as i32 });
        }
    }
    power
}

fn run_1(serial: i32, size: usize) -> (i64, Point) {
    let power = create_power_map(serial);
    run_inner(size, &power)
}

fn run_inner(size: usize, power: &Vec<Vec<i64>>) -> (i64, Point) {
    let mut max_power = 0;
    let mut max_pt = Point { x: 1, y: 1 };
    for y in 1..(301-size) {
        for x in 1..(301-size) {
            let mut sum = 0;
            for dy in y..(y+size) {
                for dx in x..(x+size) {
                    sum += power[dy-1][dx-1];
                }
            }
            if sum > max_power {
                max_power = sum;
                max_pt = Point { x: x as i32, y: y as i32 }
            }
        }
    }
    (max_power, max_pt)
}

fn run_2(serial: i32) -> (i64, usize, Point) {
    let power = create_power_map(serial);

    let sizes: Vec<usize> = (1..301).collect();

    let powers = sizes.par_iter().map(|size| (*size, run_inner(*size, &power)));

    let max = powers.max_by(|a, b| ((a.1).0.cmp(&(b.1).0))).unwrap();

    ((max.1).0, max.0, (max.1).1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc11_power_level() {
        assert_eq!(power_level(8, &Point { x: 3, y: 5 }), 4);
        assert_eq!(power_level(57, &Point { x: 122, y: 79 }), -5);
        assert_eq!(power_level(39, &Point { x: 217, y: 196 }), 0);
        assert_eq!(power_level(71, &Point { x: 101, y: 153 }), 4);
    }

    #[test]
    fn aoc11_run_1() {
        assert_eq!(run_1(18, 3),
                   (29, Point { x: 33, y: 45 }));
        assert_eq!(run_1(42, 3),
                    (30, Point { x: 21, y: 61}));
    }

    #[test]
    fn aoc11_run_2() {
        assert_eq!(run_1(18, 16),
                   (113, Point { x: 90, y: 269 }));
        assert_eq!(run_1(42, 12),
                    (119, Point { x: 232, y: 251}));

        assert_eq!(run_2(18),
                   (113, 16, Point { x: 90, y: 269 }));
        assert_eq!(run_2(42),
                    (119, 12, Point { x: 232, y: 251}));
    }

}
