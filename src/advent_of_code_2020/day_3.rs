pub fn puzzle(puzzle_input: &String, fall: u32, run: u32) -> u32 {
    let map: Vec<Vec<char>> = puzzle_input
        .trim()
        .split("\n")
        .map(|x| x.chars().collect())
        .collect();
    let height = map.len();
    let width = map[0].len();
    let mut trees = 0;
    for y in (0..height).step_by(fall as usize) {
        let x = (y as f64 / (fall as f64 / run as f64)) % width as f64;
        let space = map[y as usize][x as usize];
        if space == '#' {
            trees += 1;
        }
    }
    trees
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_test_input() {
        let test_input = fs::read_to_string(
            "resources/test/day_3/test_input.txt"
        ).unwrap();
        let x = puzzle(&test_input, 1, 3);
        assert_eq!(x, 7);
    }
    #[test]
    fn part_1_challenge_input() {
        let challenge_input = fs::read_to_string(
            "resources/test/day_3/challenge_input.txt"
        ).unwrap();
        let x = puzzle(&challenge_input, 1, 3);
        assert_eq!(x, 205);
    }

    #[test]
    fn slopes_part_2_test_input() {
        let test_input = fs::read_to_string(
            "resources/test/day_3/test_input.txt"
        ).unwrap();
        let a = puzzle(&test_input, 1, 3);
        let b = puzzle(&test_input, 1, 1);
        let c = puzzle(&test_input, 1, 5);
        let d = puzzle(&test_input, 1, 7);
        let e = puzzle(&test_input, 2, 1);
        assert_eq!(a, 7);
        assert_eq!(b, 2);
        assert_eq!(c, 3);
        assert_eq!(d, 4);
        assert_eq!(e, 2);
    }

    #[test]
    fn slopes_part_2_challenge_input() {
        let challenge_input = fs::read_to_string(
            "resources/test/day_3/challenge_input.txt"
        ).unwrap();
        let a = puzzle(&challenge_input, 1, 3);
        let b = puzzle(&challenge_input, 1, 1);
        let c = puzzle(&challenge_input, 1, 5);
        let d = puzzle(&challenge_input, 1, 7);
        let e = puzzle(&challenge_input, 2, 1);
        assert_eq!(a * b * c * d * e, 3952146825)
    }
}
