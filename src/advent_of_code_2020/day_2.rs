use regex::Regex;

pub fn part_1(puzzle_input: String) -> u32 {
    let re = Regex::new(r"(\d+)-(\d+) ([a-zA-Z]): (.+)").unwrap();
    let mut count = 0;
    for cap in re.captures_iter(&puzzle_input) {
        let min = cap[1].parse::<u32>().unwrap();
        let max = cap[2].parse::<u32>().unwrap();
        let character = cap[3].parse::<char>().unwrap();
        let password = cap[4].to_owned();
        let matches = password.matches(character).count() as u32;
        if matches >= min && matches <= max {
            count += 1
        }
    }
    count
}

pub fn part_2(puzzle_input: String) -> u32 {
    let re = Regex::new(r"(\d+)-(\d+) ([a-zA-Z]): (.+)").unwrap();
    let mut count = 0;
    for cap in re.captures_iter(&puzzle_input) {
        let first = cap[1].parse::<usize>().unwrap();
        let second = cap[2].parse::<usize>().unwrap();
        let character = cap[3].parse::<char>().unwrap();
        let password = cap[4].to_owned();
        let password_first = password.chars().nth(first - 1).unwrap();
        let password_second = password.chars().nth(second - 1).unwrap();
        if (password_first == character|| password_second == character)
                && ( password_first != password_second ) {
            count += 1;
        }
    }
    count
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn part_1_test_input() {
        let test_input = fs::read_to_string(
            "resources/test/day_2/test_input.txt"
        ).unwrap();
        let x = part_1(test_input);
        assert_eq!(x, 2);
    }

    #[test]
    fn part_1_challenge_input() {
        let challenge_input = fs::read_to_string(
            "resources/test/day_2/challenge_input.txt"
        ).unwrap();
        let x = part_1(challenge_input);
        assert_eq!(x, 398);
    }

    #[test]
    fn part_2_test_input() {
        let test_input = fs::read_to_string(
            "resources/test/day_2/test_input.txt"
        ).unwrap();
        let x = part_2(test_input);
        assert_eq!(x, 1);
    }

    #[test]
    fn part_2_challenge_input() {
        let challenge_input = fs::read_to_string(
            "resources/test/day_2/challenge_input.txt"
        ).unwrap();
        let x = part_2(challenge_input);
        assert_eq!(x, 562);
    }
}
