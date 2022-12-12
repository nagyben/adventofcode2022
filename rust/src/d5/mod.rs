use std::{collections::HashMap, hash::Hash, rc::Rc};

type Stack = Vec<char>;
type Stacks = HashMap<usize, Stack>;

use regex::Regex;

fn initialize_stacks_from_input(input: &str) -> Stacks {
    let num_stacks = get_number_of_stacks(input);
    let mut stacks: Stacks = HashMap::new();
    for i in 0..num_stacks {
        stacks.insert(i + 1, vec![]);
    }

    for line in input.split("\n").collect::<Vec<&str>>() {
        if get_input_line_type(line) == LineType::Crates {
            for i in 0..num_stacks {
                match line.chars().nth(i * 4 + 1) {
                    Some(' ') => {}
                    Some(x) => {
                        if let Some(stack) = stacks.get_mut(&(i + 1)) {
                            stack.push(x)
                        }
                    }
                    None => {}
                }
            }
        }
    }
    for (_, stack) in stacks.iter_mut() {
        stack.reverse();
    }
    stacks
}

fn get_number_of_stacks(input: &str) -> usize {
    for line in input.split("\n").collect::<Vec<&str>>() {
        if get_input_line_type(line) == LineType::StackIDs {
            return line.replace(" ", "").len();
        }
    }

    panic!("Could not figure out number of lines from input");
}

#[derive(Debug, PartialEq)]
enum LineType {
    None,
    Crates,
    StackIDs,
    Move,
}

fn get_input_line_type(input: &str) -> LineType {
    if input.len() == 0 {
        return LineType::None;
    }
    if input.contains("[") {
        return LineType::Crates;
    }
    if input.contains("move") {
        return LineType::Move;
    }
    return LineType::StackIDs;
}

#[derive(Debug, PartialEq)]
struct MoveInstruction {
    from_stack_id: usize,
    to_stack_id: usize,
    number_of_crates: usize,
}

impl MoveInstruction {
    pub fn new(from: usize, to: usize, num: usize) -> MoveInstruction {
        MoveInstruction {
            from_stack_id: from,
            to_stack_id: to,
            number_of_crates: num,
        }
    }
}

fn get_move_instruction_from_line(line: &str) -> MoveInstruction {
    let re: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let caps = re.captures(line).unwrap();
    MoveInstruction {
        from_stack_id: caps.get(2).map_or("", |m| m.as_str()).parse().unwrap(),
        to_stack_id: caps.get(3).map_or("", |m| m.as_str()).parse().unwrap(),
        number_of_crates: caps.get(1).map_or("", |m| m.as_str()).parse().unwrap(),
    }
}

fn apply_move_instruction_cratemover9000(
    stacks: &mut Stacks,
    move_instruction: MoveInstruction,
) -> () {
    for _ in 0..move_instruction.number_of_crates {
        let from = stacks.get_mut(&move_instruction.from_stack_id).unwrap();
        let c = from.pop().unwrap();
        let to = stacks.get_mut(&move_instruction.to_stack_id).unwrap();
        to.push(c);
    }
}

fn apply_move_instruction_cratemover9001(
    stacks: &mut Stacks,
    move_instruction: MoveInstruction,
) -> () {
    let mut tmp_crates: Vec<char> = vec![];
    for _ in 0..move_instruction.number_of_crates {
        let from = stacks.get_mut(&move_instruction.from_stack_id).unwrap();
        tmp_crates.push(from.pop().unwrap());
    }
    let to = stacks.get_mut(&move_instruction.to_stack_id).unwrap();
    tmp_crates.reverse();
    to.append(&mut tmp_crates)
}

fn apply_move_instruction(
    stacks: &mut Stacks,
    move_instruction: MoveInstruction,
    apply_fn: &dyn Fn(&mut Stacks, MoveInstruction) -> (),
) {
    apply_fn(stacks, move_instruction)
}

pub fn run_scenario_cratemover9000(input: &str) -> String {
    let mut stacks: Stacks = initialize_stacks_from_input(input);
    for line in input.split("\n").collect::<Vec<&str>>() {
        if get_input_line_type(line) == LineType::Move {
            let mi = get_move_instruction_from_line(line);
            apply_move_instruction(&mut stacks, mi, &apply_move_instruction_cratemover9000);
        }
    }
    let mut output = String::from("");
    for i in 0..stacks.len() {
        output.push(stacks.get(&(i + 1)).unwrap().to_owned().pop().unwrap());
    }
    output
}

pub fn run_scenario_cratemover9001(input: &str) -> String {
    let mut stacks: Stacks = initialize_stacks_from_input(input);
    for line in input.split("\n").collect::<Vec<&str>>() {
        if get_input_line_type(line) == LineType::Move {
            let mi = get_move_instruction_from_line(line);
            apply_move_instruction(&mut stacks, mi, &apply_move_instruction_cratemover9001);
        }
    }
    let mut output = String::from("");
    for i in 0..stacks.len() {
        output.push(stacks.get(&(i + 1)).unwrap().to_owned().pop().unwrap());
    }
    output
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_initialize_stacks() {
        let input = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

        let expected: Stacks = HashMap::from([
            (1, vec!['Z', 'N']),
            (2, vec!['M', 'C', 'D']),
            (3, vec!['P']),
        ]);

        let actual = initialize_stacks_from_input(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_number_of_stacks() {
        let input = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

        let actual = get_number_of_stacks(input);

        assert_eq!(actual, 3);
    }

    #[rstest]
    #[case("    [D]    ", LineType::Crates)]
    #[case("[N] [C]    ", LineType::Crates)]
    #[case("[Z] [M] [P]", LineType::Crates)]
    #[case(" 1   2   3 ", LineType::StackIDs)]
    #[case("", LineType::None)]
    #[case("move 1 from 2 to 1", LineType::Move)]
    fn test_get_input_line_type(#[case] input: &str, #[case] expected: LineType) {
        let actual = get_input_line_type(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_run_scenario_cratemover9000() {
        let input = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

        let expected: &str = "CMZ";

        let actual = run_scenario_cratemover9000(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_run_scenario_cratemover9001() {
        let input = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

        let expected: &str = "MCD";

        let actual = run_scenario_cratemover9001(input);

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("move 1 from 1 to 2", MoveInstruction::new(1, 2, 1))]
    #[case("move 2 from 2 to 3", MoveInstruction::new(2, 3, 2))]
    fn test_get_move_instruction_from_line(#[case] input: &str, #[case] expected: MoveInstruction) {
        let actual = get_move_instruction_from_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_apply_move_instruction_cratemover9000_case1() {
        let mut stacks = Stacks::from([
            (1, vec!['Z', 'N']),
            (2, vec!['M', 'C', 'D']),
            (3, vec!['P']),
        ]);
        let mi = MoveInstruction::new(2, 1, 1);

        apply_move_instruction_cratemover9000(&mut stacks, mi);

        assert_eq!(
            stacks,
            Stacks::from([
                (1, vec!['Z', 'N', 'D']),
                (2, vec!['M', 'C']),
                (3, vec!['P']),
            ])
        );
    }

    #[test]
    fn test_apply_move_instruction_cratemover9000_case2() {
        let mut stacks = Stacks::from([
            (1, vec!['Z', 'N', 'D']),
            (2, vec!['M', 'C']),
            (3, vec!['P']),
        ]);
        let mi = MoveInstruction::new(1, 3, 3);

        apply_move_instruction(&mut stacks, mi, &apply_move_instruction_cratemover9000);

        assert_eq!(
            stacks,
            Stacks::from([
                (1, vec![]),
                (2, vec!['M', 'C']),
                (3, vec!['P', 'D', 'N', 'Z']),
            ])
        );
    }

    #[test]
    fn test_apply_move_instruction_cratemover9001_case1() {
        let mut stacks = Stacks::from([
            (1, vec!['Z', 'N', 'D']),
            (2, vec!['M', 'C']),
            (3, vec!['P']),
        ]);
        let mi = MoveInstruction::new(1, 3, 3);

        apply_move_instruction(&mut stacks, mi, &apply_move_instruction_cratemover9001);

        assert_eq!(
            stacks,
            Stacks::from([
                (1, vec![]),
                (2, vec!['M', 'C']),
                (3, vec!['P', 'Z', 'N', 'D']),
            ])
        );
    }
}
