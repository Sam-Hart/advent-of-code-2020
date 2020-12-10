use regex::Regex;
use std::collections::{ HashMap, HashSet };

#[derive(Debug)]
struct HeldBag {
    held: u32,
    color: String,
}

fn parse_input(input: &String) -> HashMap<String, Vec<HeldBag>> {
    let bags_re = Regex::new(
        r"([\w]* [\w]*) bags contain| ((\d*) ([\w]* [\w]*) bag[s]?[,.])"
    ).unwrap();
    let mut bags: HashMap<String, Vec<HeldBag>> = HashMap::new();
    let mut last_specified_root: &str = "";
    for cap in bags_re.captures_iter(&input.trim()) {
        let root_color = cap.get(1);
        match root_color {
            Some(color) => {
                last_specified_root = color.as_str();
                continue;
            },
            _ => ()
        }
        let held_bag = HeldBag {
            held: cap.get(3).map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
            color: cap.get(4).map_or("".to_owned(), |m| m.as_str().to_owned()),
        };
        let contains = bags
            .entry(last_specified_root.to_owned())
            .or_insert(Vec::new());
        contains.push(held_bag);
    }
    return bags;
}

fn get_to_bag(
    bag_descriptors: &HashMap<String, Vec<HeldBag>>,
    bag_to_find: &str
) -> HashSet<String> {
    let mut able_to_contain: HashSet<String> = HashSet::new();
    for (container, _contains) in bag_descriptors {
        if *container == bag_to_find.to_owned() {
            continue;
        }
        if drill_into(bag_descriptors, container, bag_to_find) {
            able_to_contain.insert(container.clone());
        }
    }
    able_to_contain
}

fn drill_into(
    bag_descriptors: &HashMap<String, Vec<HeldBag>>,
    drilled: &String,
    bag_to_find: &str,
) -> bool {
    let drilled_bag_contents = bag_descriptors.get(drilled);
    if drilled_bag_contents.is_none() {
        return false;
    }
    for held_bag in drilled_bag_contents.unwrap() {
        if *held_bag.color == bag_to_find.to_owned() {
            return true
        }
        if drill_into(bag_descriptors, &held_bag.color, bag_to_find) {
            return true
        }
    }
    false
}

pub fn part_1(puzzle_input: String) -> u32 {
    let bag_descriptors = parse_input(&puzzle_input);
    let bag_to_find = "shiny gold";

    let containing_bags = get_to_bag(&bag_descriptors, bag_to_find);

    containing_bags.len() as u32
}

fn get_bags(
    bag_descriptors: &HashMap<String, Vec<HeldBag>>,
    outer_bag: &String
) -> u32 {
    let held_bags = bag_descriptors.get(outer_bag);
    if held_bags.is_none() {
        return 0
    }
    let mut sum = 0;
    for held_bag in held_bags.unwrap() {
        sum += held_bag.held;
        sum += get_bags(bag_descriptors, &held_bag.color) * held_bag.held;
    }
    sum
}

pub fn part_2(puzzle_input: String) -> u32 {
    let bag_descriptors = parse_input(&puzzle_input);
    let outer_bag = String::from("shiny gold");
    let bags_to_hold = get_bags(&bag_descriptors, &outer_bag);
    bags_to_hold
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_test_input() {
        let test_input = fs::read_to_string(
            "resources/test/day_7/test_input.txt"
        ).unwrap();

        let x = part_1(test_input);
        assert_eq!(x, 4);
    }

    #[test]
    fn part_1_challenge_input() {
        let challenge_input = fs::read_to_string(
            "resources/test/day_7/challenge_input.txt"
        ).unwrap();
        let x = part_1(challenge_input);
        assert_eq!(x, 155);
    }

    #[test]
    fn part_2_test_input_2() {
        let test_input = fs::read_to_string(
            "resources/test/day_7/test_input_2.txt"
        ).unwrap();

        let x = part_2(test_input);
        assert_eq!(x, 126);
    }

    #[test]
    fn part_2_challenge_input() {
        let challenge_input = fs::read_to_string(
            "resources/test/day_7/challenge_input.txt"
        ).unwrap();

        let x = part_2(challenge_input);
        assert_eq!(x, 54803);
    }
}
