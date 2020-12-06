use std::collections::HashSet;
pub fn part_1(puzzle_input: String) -> u32 {
    let total_answers: u32 = puzzle_input
        .trim()
        .split("\n\n")
        .fold(0, |acc, group| {
            let group_characters: HashSet<char> = group
                .split("\n")
                .fold(HashSet::new(), |mut acc, person| {
                    for answer in person.chars() {
                        acc.insert(answer);
                    }
                    acc
                });
            acc + group_characters.len() as u32
        });
    total_answers
}

pub fn part_2(puzzle_input: String) -> u32 {
    let total_answers: u32 = puzzle_input
        .trim()
        .split("\n\n")
        .fold(0, |acc, group| {
            let group_characters: Vec<&str> = group
                .split("\n")
                .collect();
            let mut initial_set = HashSet::new();
            for answer in group_characters[0].chars() {
                initial_set.insert(answer);
            }
            let group_characters: HashSet<char> = group
                .split("\n")
                .fold(initial_set, |acc, person| {
                    let mut candidate_answers = HashSet::new();
                    for answer in person.chars() {
                        candidate_answers.insert(answer);
                    }
                    acc.intersection(&candidate_answers).copied().collect()
                });
            acc + group_characters.len() as u32
        });
    total_answers
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_test_input() {
        let test_input = fs::read_to_string(
            "resources/test/day_6/test_input.txt"
        ).unwrap();
        let x = part_1(test_input);
        assert_eq!(x, 11);
    }

    #[test]
    fn part_1_challenge_input() {
        let challenge_input = fs::read_to_string(
            "resources/test/day_6/challenge_input.txt"
        ).unwrap();
        let x = part_1(challenge_input);
        assert_eq!(x, 6351);
    }

    #[test]
    fn part_2_test_input() {
        let test_input = fs::read_to_string(
            "resources/test/day_6/test_input.txt"
        ).unwrap();
        let x = part_2(test_input);
        assert_eq!(x, 6);
    }

    #[test]
    fn part_2_challenge_input() {
        let challenge_input = fs::read_to_string(
            "resources/test/day_6/challenge_input.txt"
        ).unwrap();
        let x = part_2(challenge_input);
        assert_eq!(x, 3143);
    }
}
