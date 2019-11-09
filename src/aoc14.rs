#[derive(Debug)]
struct State {
    elf_1_pos: usize,
    elf_2_pos: usize,
    recipies: Vec<u8>,
}

impl State {
    fn new() -> Self {
        Self {
            elf_1_pos: 0,
            elf_2_pos: 1,
            recipies: vec![3, 7],
        }
    }

    fn advance(&mut self) {
        let sum = self.recipies[self.elf_1_pos] + self.recipies[self.elf_2_pos];

        self.recipies.append(&mut sum_to_vec(sum));

        self.elf_1_pos =
            (self.elf_1_pos + self.recipies[self.elf_1_pos] as usize + 1) % self.recipies.len();
        self.elf_2_pos =
            (self.elf_2_pos + self.recipies[self.elf_2_pos] as usize + 1) % self.recipies.len();
    }
}

pub fn run() {
    println!("14:1 - {}", run_1(990941));
    println!("14:2 - {}", run_2(990941));
}

fn run_1(num_recipies: usize) -> usize {
    let mut state = State::new();
    while state.recipies.len() < (num_recipies + 10) {
        state.advance();
    }
    score(&state.recipies, num_recipies)
}

fn run_2(mut int_recipies: usize) -> usize {
    let mut pattern = Vec::new();

    while int_recipies > 0 {
        pattern.push((int_recipies % 10) as u8);
        int_recipies /= 10;
    }
    pattern.reverse();

    let mut state = State::new();

    while state.recipies.len() < pattern.len() {
        state.advance();
    }

    let mut search_idx = 0;

    loop {
        let cur_len = state.recipies.len();
        state.advance();
        let delta_len = state.recipies.len() - cur_len;

        for _ in 0..delta_len {
            let pos = state.recipies[search_idx..]
                .windows(pattern.len())
                .position(|p| p == pattern.as_slice());

            if let Some(idx) = pos {
                return search_idx + idx;
            }
            search_idx += 1;
        }
    }
}

fn sum_to_vec(sum: u8) -> Vec<u8> {
    if sum < 10 {
        vec![sum]
    } else {
        vec![sum / 10, sum % 10]
    }
}

fn score(recipies: &Vec<u8>, num_recipies: usize) -> usize {
    recipies[num_recipies..num_recipies + 10]
        .iter()
        .fold(0, |acc, score| (acc * 10) + *score as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc14_sum_to_recipe() {
        assert_eq!(vec![1], sum_to_vec(1));
        assert_eq!(vec![8], sum_to_vec(8));
        assert_eq!(vec![1, 1], sum_to_vec(11));
        assert_eq!(vec![1, 8], sum_to_vec(18));
    }

    #[test]
    fn aoc14_advance() {
        let mut state = State::new();
        assert_eq!(vec![3, 7], state.recipies);
        state.advance();
        assert_eq!((0, 1), (state.elf_1_pos, state.elf_2_pos));
        assert_eq!(vec![3, 7, 1, 0], state.recipies);
        state.advance();
        assert_eq!((4, 3), (state.elf_1_pos, state.elf_2_pos));
        assert_eq!(vec![3, 7, 1, 0, 1, 0], state.recipies);
        state.advance();
        assert_eq!((6, 4), (state.elf_1_pos, state.elf_2_pos));
        assert_eq!(vec![3, 7, 1, 0, 1, 0, 1], state.recipies);
        state.advance();
        assert_eq!((0, 6), (state.elf_1_pos, state.elf_2_pos));
        assert_eq!(vec![3, 7, 1, 0, 1, 0, 1, 2], state.recipies);
        state.advance();
        assert_eq!((4, 8), (state.elf_1_pos, state.elf_2_pos));
        assert_eq!(vec![3, 7, 1, 0, 1, 0, 1, 2, 4], state.recipies);

        (0..10).for_each(|_| state.advance());

        assert_eq!((8, 4), (state.elf_1_pos, state.elf_2_pos));
        assert_eq!(
            vec![3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5, 8, 9, 1, 6, 7, 7, 9, 2],
            state.recipies
        );
    }

    #[test]
    fn aoc14_score() {
        assert_eq!(9876543210, score(&vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0], 0));
        assert_eq!(123456789, score(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 0));
        assert_eq!(
            5158916779,
            score(
                &vec![3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5, 8, 9, 1, 6, 7, 7, 9, 2],
                9
            )
        );
    }

    #[test]
    fn aoc14_run_1() {
        assert_eq!(5158916779, run_1(9));
        assert_eq!(0124515891, run_1(5));
        assert_eq!(9251071085, run_1(18));
        assert_eq!(5941429882, run_1(2018));
    }

    #[test]
    fn aoc14_run_2() {
        assert_eq!(9, run_2(51589));
        assert_eq!(6, run_2(01245));
        assert_eq!(18, run_2(92510));
        assert_eq!(2018, run_2(59414));
    }
}
