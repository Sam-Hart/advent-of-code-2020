use std::collections::HashMap;
use regex::Regex;
pub fn part_1(puzzle_input: String) -> u32 {
    let passports: Vec<&str> = puzzle_input
        .trim()
        .split("\n\n")
        .filter(|passport| {
            let string_passport = passport.to_string();
            string_passport.contains("byr") &&
            string_passport.contains("iyr") &&
            string_passport.contains("eyr") &&
            string_passport.contains("hgt") &&
            string_passport.contains("hcl") &&
            string_passport.contains("ecl") &&
            string_passport.contains("pid")
        })
        .collect();
    passports.len() as u32
}

pub fn part_2(puzzle_input: String) -> u32 {
    // : Vec<HashMap<&str, &str>>
    let passports: Vec<HashMap<&str, &str>> = puzzle_input
        .trim()
        .split("\n\n")
        .map(|passport| {
            let fields: Vec<&str> = passport
                .split("\n") // vec<some fields>
                .map(|line| line.split(" "))
                .flatten()
                .collect();
            let mut mapped_fields: HashMap<&str, &str> = HashMap::new();
            for field in fields {
                let key_value_pair: Vec<&str> = field.split(":").collect();
                mapped_fields.insert(
                    key_value_pair[0],
                    key_value_pair[1]
                );
            }
            mapped_fields
        })
        .collect();
    let valid_passports: Vec<HashMap<&str, &str>> = passports
        .into_iter()
        .filter(|passport| {
            !passport.get("byr").is_none() &&
            !passport.get("iyr").is_none() &&
            !passport.get("eyr").is_none() &&
            !passport.get("hgt").is_none() &&
            !passport.get("hcl").is_none() &&
            !passport.get("ecl").is_none() &&
            !passport.get("pid").is_none()
        })
        .filter(|present| {
            // TODO: Should be refactored to not be such a large block
            // Each validation set for each individual rule could be its own
            // function pulled out - possibly use a struct instead of a
            // HashMap as well
            let height_re = Regex::new(
                "^(1[5-8][0-9]|19[0-3])cm$|^(59|6[0-9]|7[0-6])in$"
            ).unwrap();
            let color_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            let eye_re = Regex::new(
                r"^amb$|^blu$|^brn$|^gry$|^grn$|^hzl$|^oth$"
            ).unwrap();
            let pid_re = Regex::new("^[0-9]{9}$").unwrap();
            let year_re = Regex::new("^[0-9]{4}$").unwrap();

            let birth = present.get("byr").unwrap();
            let issue = present.get("iyr").unwrap();
            let expiration = present.get("eyr").unwrap();
            let height = present.get("hgt").unwrap();
            let hair = present.get("hcl").unwrap();
            let eye = present.get("ecl").unwrap();
            let pid = present.get("pid").unwrap();

            let birth_year = birth.parse::<i32>().unwrap_or(0);
            let issue_year = issue.parse::<i32>().unwrap_or(0);
            let expiration_year = expiration.parse::<i32>().unwrap_or(0);
            let valid_eye_color = eye_re.is_match(eye);
            let valid_hair_color = color_re.is_match(hair);
            let valid_pid = pid_re.is_match(pid);
            let valid_height = height_re.is_match(height);
            let valid_birth = birth_year >= 1920 && birth_year <= 2002
                && year_re.is_match(birth);
            let valid_expiration = expiration_year >= 2020
                && expiration_year <= 2030
                && year_re.is_match(expiration);
            let valid_issue = issue_year >= 2010 && issue_year <= 2020
                && year_re.is_match(issue);
            valid_eye_color &&
            valid_hair_color &&
            valid_pid &&
            valid_height &&
            valid_birth &&
            valid_expiration &&
            valid_issue
        })
        .collect();

    valid_passports.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_test_input() {
        let test_input = fs::read_to_string(
            "resources/test/day_4/test_input.txt"
        ).unwrap();
        let x = part_1(test_input);
        assert_eq!(x, 2);
    }

    #[test]
    fn part_1_challenge_input() {
        let challenge_input = fs::read_to_string(
            "resources/test/day_4/challenge_input.txt"
        ).unwrap();
        let x = part_1(challenge_input);
        assert_eq!(x, 190);
    }

    #[test]
    fn part_2_test_input() {
        let test_input = fs::read_to_string(
            "resources/test/day_4/test_input_2.txt"
        ).unwrap();
        let x = part_2(test_input);
        assert_eq!(x, 4);
    }

    #[test]
    fn part_2_challenge_input() {
        let challenge_input = fs::read_to_string(
            "resources/test/day_4/challenge_input.txt"
        ).unwrap();
        let x = part_2(challenge_input);
        assert_eq!(x, 121);
    }
}
