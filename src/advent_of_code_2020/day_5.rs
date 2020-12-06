#[derive(Debug, )]
struct BoardingPass {
    id: u32,
    row: u32,
    column: u32,
}

fn binary_search(instructions: &str, up: char, down: char) -> u32 {
    let base: u32 = 2;
    let mut min = 0;
    let mut max = base.pow(instructions.len() as u32) - 1;
    for instruction in instructions.chars() {
        let difference = max - min;
        match instruction {
            x if x == up => {
                min = ((difference + (difference % 2)) / 2) + min;
            },
            x if x == down => {
                max = max - (difference / 2);
            },
            _ => ()
        }
    }
    min
}

fn calculate_boarding_pass(pass: String) -> BoardingPass {
    let row_descriptor = &pass[0..7];
    let column_descriptor = &pass[7..];
    let row = binary_search(row_descriptor, 'B', 'F');
    let column = binary_search(column_descriptor, 'R', 'L');
    BoardingPass {
        id: (row * 8) + column,
        row: row,
        column: column,
    }
}

pub fn part_1(puzzle_input: String) -> u32 {
    let boarding_passes: Vec<BoardingPass> = puzzle_input
        .trim()
        .split("\n")
        .map(|instructions| calculate_boarding_pass(instructions.to_string()))
        .collect();

    let max_id: u32 = boarding_passes
        .into_iter()
        .fold(
            0,
            |acc, pass| {
                if pass.id > acc {
                    pass.id
                } else {
                    acc
                }
            }
        );
    max_id
}

pub fn part_2(puzzle_input: String) -> u32 {
    let mut boarding_passes: Vec<BoardingPass> = puzzle_input
        .trim()
        .split("\n")
        .map(|instructions| calculate_boarding_pass(instructions.to_string()))
        .collect();

    boarding_passes.sort_by_key(|d| d.id);
    for i in 1..boarding_passes.len() - 1 {
        let candidate_pass = &boarding_passes[i];
        let next_pass = &boarding_passes[i + 1];
        if candidate_pass.id < next_pass.id - 1 {
            return candidate_pass.id + 1
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn binary_search_returns_0_for_all_down() {
        let x = binary_search("DDDDDDDDDDDDDDD", 'U', 'D');
        assert_eq!(x, 0);
    }

    #[test]
    fn binary_search_returns_32_for_all_up() {
        let x = binary_search("UUUUU", 'U', 'D');
        assert_eq!(x, 31);
    }

    #[test]
    fn test_binary_boarding() {
        let test_input = "BFFFBBFRRR".to_string();
        let x = calculate_boarding_pass(test_input);
        assert_eq!(x.row, 70);
        assert_eq!(x.column, 7);
        assert_eq!(x.id, 567);
    }

    #[test]
    fn part_1_test_input() {
        let test_input = fs::read_to_string(
            "resources/test/day_5/test_input.txt"
        ).unwrap();
        let x = part_1(test_input);
        assert_eq!(x, 820);
    }

    #[test]
    fn part_1_challenge_input() {
        let challenge_input = fs::read_to_string(
            "resources/test/day_5/challenge_input.txt"
        ).unwrap();
        let x = part_1(challenge_input);
        assert_eq!(x, 826);
    }

    #[test]
    fn part_2_challenge_input() {
        let challenge_input = fs::read_to_string(
            "resources/test/day_5/challenge_input.txt"
        ).unwrap();
        let x = part_2(challenge_input);
        assert_eq!(x, 678);
    }
}
