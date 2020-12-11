use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
struct StateModifier {
    accumulator_change: i32,
    stack_pointer_change: i32,
}
#[derive(Clone, Debug)]
struct Instruction {
    op_code: String,
    value: i32,
}

struct Computer {
    instructions: Vec<Instruction>,
    stack_pointer: i32,
    accumulator: i32,
}

impl Computer {
    fn new(program: &String) -> Computer {
        let instruction_re = Regex::new(r"([\w]{3}) ([+-][\d]+)").unwrap();
        let mut instructions: Vec<Instruction> = Vec::new();
        for cap in instruction_re.captures_iter(&program.trim()) {
            let op_code = String::from(cap.get(1).unwrap().as_str());
            let value = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
            instructions.push(Instruction {
                op_code,
                value,
            })
        }
        Computer {
            instructions,
            stack_pointer: 0,
            accumulator: 0,
        }
    }

    fn compute(&mut self) -> i32 {
        let mut executed_lines: HashSet<i32> = HashSet::new();
        while !executed_lines.contains(&self.stack_pointer)
            && self.stack_pointer < self.instructions.len() as i32
        {
            executed_lines.insert(self.stack_pointer);
            let next = &self.instructions[self.stack_pointer as usize];
            let executor = find_executor(&next.op_code);
            let modification = executor(
                &next.value,
                &self.stack_pointer,
                &self.accumulator
            );
            self.stack_pointer += modification.stack_pointer_change;
            self.accumulator += modification.accumulator_change;
        }
        self.accumulator
    }

    fn compute_corrupted(&mut self) -> i32 {
        let pristine_instructions = self.instructions.clone();
        for i in 0..pristine_instructions.len() {
            let instruction = &pristine_instructions[i];
            let new_op = match instruction.op_code.as_str() {
                "jmp" => "nop",
                "nop" => "jmp",
                x => x,
            };
            self.instructions[i] = Instruction {
                op_code: String::from(new_op),
                value: instruction.value
            };
            let result = self.compute();
            if self.stack_pointer >= pristine_instructions.len() as i32 {
                return result
            }
            self.instructions[i] = pristine_instructions[i].clone();
            self.stack_pointer = 0;
            self.accumulator = 0;
        }
        0
    }
}

type InstructionExecutor = fn(&i32, &i32, &i32) -> StateModifier;
fn find_executor(op_code: &String) -> InstructionExecutor {
    match op_code.as_str() {
        "nop" => no_op,
        "jmp" => jump,
        "acc" => accumulate,
        _ => unimplemented!()
    }
}

fn no_op(_value: &i32, _pointer: &i32, _acc: &i32) -> StateModifier {
    StateModifier {
        stack_pointer_change: 1,
        accumulator_change: 0,
    }
}

fn jump(value: &i32, _pointer: &i32, _acc: &i32) -> StateModifier {
    StateModifier {
        stack_pointer_change: *value,
        accumulator_change: 0,
    }
}

fn accumulate(value: &i32, _pointer: &i32, _acc: &i32) -> StateModifier {
    StateModifier {
        stack_pointer_change: 1,
        accumulator_change: *value,
    }
}

pub fn part_1(puzzle_input: String) -> i32 {
    let mut computer = Computer::new(&puzzle_input);
    computer.compute()
}

pub fn part_2(puzzle_input: String) -> i32 {
    let mut computer = Computer::new(&puzzle_input);
    computer.compute_corrupted()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_test_input() {
        let test_input = fs::read_to_string(
            "resources/test/day_8/test_input.txt"
        ).unwrap();
        let x = part_1(test_input);
        assert_eq!(x, 5);
    }

    #[test]
    fn part_1_challenge_input() {
        let challenge_input = fs::read_to_string(
            "resources/test/day_8/challenge_input.txt"
        ).unwrap();
        let x = part_1(challenge_input);
        assert_eq!(x, 1528);
    }

    #[test]
    fn part_2_test_input() {
        let test_input = fs::read_to_string(
            "resources/test/day_8/test_input_2.txt"
        ).unwrap();
        let x = part_2(test_input);
        assert_eq!(x, 8);
    }

    #[test]
    fn part_2_challenge_input() {
        let challenge_input = fs::read_to_string(
            "resources/test/day_8/challenge_input.txt"
        ).unwrap();
        let x = part_2(challenge_input);
        assert_eq!(x, 640);
    }
}
