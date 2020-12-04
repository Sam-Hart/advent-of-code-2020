use std::collections::HashSet;
pub fn part_1(puzzle_input: String) -> Result<i32, ()> {
    let integers: HashSet<i32> = puzzle_input
        .trim()
        .split("\n")
        .into_iter()
        .map(|x| x.parse().unwrap())
        .collect();

    for key in &integers {
        let addend = 2020 - key;
        if integers.contains(&addend) {
            return Ok(key * addend);
        }
    }
    Err(())
}

pub fn part_2(puzzle_input: String) -> Result<i32, ()> {
    let integers: HashSet<i32> = puzzle_input
        .trim()
        .split("\n")
        .into_iter()
        .map(|x| x.parse().unwrap())
        .collect();

    for a in &integers {
        let bc_sum = 2020 - a;
        for b in &integers {
            let c = bc_sum - b;
            if integers.contains(&b) && integers.contains(&c) {
                return Ok(a * b * c);
            }
        }
    }
    Err(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn part_1_test_input() -> Result<(), ()> {
        let test_input = fs::read_to_string(
            "resources/test/day_1/test_input.txt"
        ).unwrap();
        let x = part_1(test_input)?;
        assert_eq!(x, 514579);
        Ok(())
    }
    #[test]
    fn part_1_challenge_input() -> Result<(), ()> {
        let challenge_input = fs::read_to_string(
            "resources/test/day_1/challenge_input.txt"
        ).unwrap();
        let x = part_1(challenge_input)?;
        assert_eq!(x, 224436);
        Ok(())
    }

    #[test]
    fn part_2_test_input() -> Result<(), ()> {
        let test_input = fs::read_to_string(
            "resources/test/day_1/test_input.txt"
        ).unwrap();
        let x = part_2(test_input)?;
        assert_eq!(x, 241861950);
        Ok(())
    }

    #[test]
    fn part_2_challenge_input() -> Result<(), ()> {
        let challenge_input = fs::read_to_string(
            "resources/test/day_1/challenge_input.txt"
        ).unwrap();
        let x = part_2(challenge_input)?;
        assert_eq!(x, 303394260);
        Ok(())
    }
}
