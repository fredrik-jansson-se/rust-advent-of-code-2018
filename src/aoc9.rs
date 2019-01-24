pub fn run() {
    println!("day9-1: {}", run_1(435, 71184));
    println!("day9-2: {}", run_1(435, 71184 * 100));
}

fn run_1(no_players: usize, last_marble: usize) -> usize {
    let mut scores = vec![0; no_players];
    let mut marbles = vec![0; 1];
    let mut cur_idx = 0;
    let mut current_player = 0;

    for cur in 1..(last_marble + 1) {
        if cur % 23 == 0 {
            if cur_idx < 7 {
                cur_idx = cur_idx + marbles.len() - 7;
            } else {
                cur_idx -= 7;
            }
            scores[current_player] += cur + marbles[cur_idx];
            marbles.remove(cur_idx);
        } else {
            cur_idx = (cur_idx + 1) % marbles.len() + 1;
            marbles.insert(cur_idx, cur);
        }
        current_player = (current_player + 1) % no_players;
        // dbg!(&marbles);
        if cur % 1000 == 0 {
            dbg!((cur, marbles.len()));
        }
    }

    *scores.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc9_run_1() {
        assert_eq!(run_1(10, 25), 32);
        assert_eq!(run_1(10, 1618), 8317);
        assert_eq!(run_1(13, 7999), 146373);
        assert_eq!(run_1(17, 1104), 2764);
        assert_eq!(run_1(21, 6111), 54718);
        assert_eq!(run_1(30, 5807), 37305);
    }
}
