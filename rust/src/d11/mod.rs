use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{digit1, line_ending, multispace0, space0},
    combinator::{map, map_res},
    multi::many0,
    sequence::pair,
    IResult,
};

use std::{
    cell::{Ref, RefCell},
    collections::{HashMap, VecDeque},
    fmt,
    rc::Rc,
};

const NUM_ROUNDS: usize = 20;
trait Parseable {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}
#[derive(Debug, PartialEq, Clone)]
enum Test {
    DivisibleBy(u64),
}

impl Parseable for Test {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = multispace0(input)?;
        let (input, _) = tag("Test: ")(input)?;
        let (input, test) = map(tag("divisible by "), |_| Test::DivisibleBy)(input)?;
        let (input, divisible_by_number) = nom::character::complete::u64(input)?;

        Ok((input, test(divisible_by_number)))
    }
}
#[derive(Debug, PartialEq, Clone)]
struct Monkey {
    number: usize,
    items: VecDeque<Item>,
    operation: Operation,
    test: Test,
    test_result_monkey: TestResultTargetMonkey,
    num_inspected: usize,
}
impl Parseable for Monkey {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let (input, _) = multispace0(input)?;
        println!("{:?}", input);
        let (input, _) = tag("Monkey ")(input)?;
        let (input, monkey_number) = nom::character::complete::u64(input)?;
        let (input, _) = take_until("\n")(input)?;
        let (input, _) = line_ending(input)?;
        let (input, _) = space0(input)?;
        let (input, _) = tag("Starting items: ")(input)?;
        let (input, items) = nom::multi::separated_list1(tag(", "), Item::parse)(input)?;
        let (input, _) = line_ending(input)?;
        let (input, _) = space0(input)?;
        let (input, _) = tag("Operation: new = old ")(input)?;
        let (input, operation) = alt((
            // map(tag("* "), |_| -> Box<dyn Fn(u64) -> Operation> {
            //     Box::new(|val| Operation::Multiply(val))
            // }),
            // map(tag("+ "), |_| -> Box<dyn Fn(u64) -> Operation> {
            //     Box::new(|val| Operation::Add(val))
            // }),
            // map(tag("* old"), |_| -> Box<dyn Fn(u64) -> Operation> {
            //     Box::new(|_| Operation::Square)
            // }),
            map(pair(tag("+ "), map_res(digit1, str::parse)), |(_, val)| {
                Operation::Add(val)
            }),
            map(pair(tag("* "), map_res(digit1, str::parse)), |(_, val)| {
                Operation::Multiply(val)
            }),
            map(tag("* old"), |_| Operation::Square),
        ))(input)?;
        let (input, test) = Test::parse(input)?;
        let (input, test_result_monkey) = TestResultTargetMonkey::parse(input)?;
        Ok((
            input,
            Monkey {
                number: monkey_number as usize,
                items: items.into(),
                operation: operation,
                test: test,
                test_result_monkey: test_result_monkey,
                num_inspected: 0,
            },
        ))
    }
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Monkey {}: {}",
            self.number,
            self.items
                .iter()
                .map(|item| item.worry.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}

type Monkeys = Vec<Monkey>;

#[derive(Debug, PartialEq, Clone)]
struct Item {
    worry: u64,
}

impl Parseable for Item {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = multispace0(input)?;
        let (input, worry) = nom::character::complete::u64(input)?;
        Ok((input, Item { worry }))
    }
}
#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

#[derive(Debug, PartialEq, Clone)]
struct TestResultTargetMonkey {
    when_true: usize,
    when_false: usize,
}
impl Parseable for TestResultTargetMonkey {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = multispace0(input)?;
        let (input, _) = tag("If true: throw to monkey ")(input)?;
        let (input, when_true) = nom::character::complete::u64(input)?;
        let (input, _) = line_ending(input)?;
        let (input, _) = space0(input)?;
        let (input, _) = tag("If false: throw to monkey ")(input)?;
        let (input, when_false) = nom::character::complete::u64(input)?;
        Ok((
            input,
            TestResultTargetMonkey {
                when_true: when_true as usize,
                when_false: when_false as usize,
            },
        ))
    }
}

fn parse_monkeys(input: &str) -> Result<Monkeys, Box<dyn std::error::Error + '_>> {
    let (input, monkeys) = many0(Monkey::parse)(input)?;
    return Ok(monkeys);
}

fn print_monkeys(monkeys: &Monkeys) {
    for monkey in monkeys.iter() {
        println!("{}", monkey);
    }
}

pub fn simulation(input: &str, num_rounds: u64, worry_divisor: u64) -> u64 {
    let mut monkeys = parse_monkeys(input).unwrap();
    println!("-------- Round 0 ---------");
    // print_monkeys(&monkeys);
    for round_number in 1..=num_rounds {
        // println!("-------- Round {} ---------", round_number);
        round(&mut monkeys, worry_divisor);
        // print_monkeys(&monkeys);
    }
    let mut sorted_inspections = monkeys
        .iter()
        .map(|monkey| monkey.num_inspected)
        .collect::<Vec<_>>();
    sorted_inspections.sort();
    sorted_inspections.reverse();
    println!("{:?}", sorted_inspections);
    (sorted_inspections[0] as u64 * sorted_inspections[1] as u64) as u64
}

fn round(monkeys: &mut Monkeys, worry_divisor: u64) {
    // for part 2 this requires some modulo math in order to keep the worry number low enough to fit
    // into a 32- (or even 64-) bit uint
    // the trick here is to find the least common multiple of all the monkeys' test numbers
    // and then modulo the worry number by that
    // The reason this works is because the test is always a modulo test, so if the worry number
    // is divisible by the test number, then the worry number modulo the LCM will also be the same

    // e.g.
    // divisor tests: 3, 5, 7
    // item worry level: 123645
    // LCM of 3, 5, 7 = 105
    // 123645 % 3 = 0
    // 123645 % 5 = 0
    // 123645 % 7 = 4

    // mod the big number by the LCM of all the divisors
    // 123645 % 105 = 60
    // 60 % 3 = 0
    // 60 % 5 = 0
    // 60 % 7 = 4
    // since we get the same remainder, we can use this trick to keep the worry number small
    let lcm_divisor = monkeys.iter().fold(1, |acc, monkey| match monkey.test {
        Test::DivisibleBy(val) => acc * val,
    });
    for i in 0..monkeys.len() {
        let mc;
        {
            let monkey = &mut monkeys[i];
            mc = monkey.clone();
            monkey.num_inspected += monkey.items.len();
        }
        while let Some(item) = monkeys[i].items.pop_front() {
            let mut new_item = item.clone();
            let new_item_clone = new_item.clone();
            new_item.worry = match &mc.operation {
                Operation::Add(val) => item.worry + val,
                Operation::Multiply(val) => item.worry * val,
                Operation::Square => item.worry * item.worry,
            } % lcm_divisor
                / worry_divisor;
            match mc.test {
                Test::DivisibleBy(val) => {
                    let destination_monkey: usize;
                    if new_item.worry % val == 0 {
                        destination_monkey = mc.test_result_monkey.when_true;
                    } else {
                        destination_monkey = mc.test_result_monkey.when_false;
                    }
                    monkeys[destination_monkey].items.push_back(new_item);

                    // println!(
                    //     "Monkey {} inspected item with worry {}. New item is {}. Throwing to monkey {}",
                    //     mc.number,
                    //     item.worry,
                    //     new_item_clone.worry,
                    //     destination_monkey
                    // );
                }
            }
        }
    }
}

pub fn part1(input: &str) -> u64 {
    simulation(input, 20, 3)
}
pub fn part2(input: &str) -> u64 {
    simulation(input, 10000, 1)
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;

    #[test]
    fn test_parse_test() {
        let input = "  Test: divisible by 3";
        let expected = Test::DivisibleBy(3);
        let (_, test) = Test::parse(input).unwrap();
        assert_eq!(test, expected);
    }
    #[test]
    fn parse_TestResultTargetMonkey() {
        let input = r#"    If true: throw to monkey 2
    If false: throw to monkey 3"#;
        let expected = TestResultTargetMonkey {
            when_true: 2,
            when_false: 3,
        };
        let (_, test_result_monkey) = TestResultTargetMonkey::parse(input).unwrap();
        assert_eq!(test_result_monkey, expected);
    }

    #[test]
    fn test_parse_monkey() {
        let input = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 4"#;

        let expected = Monkey {
            number: 0,
            num_inspected: 0,
            items: VecDeque::from(vec![Item { worry: 79 }, Item { worry: 98 }]),
            operation: Operation::Multiply(19),
            test: Test::DivisibleBy(23),
            test_result_monkey: TestResultTargetMonkey {
                when_true: 2,
                when_false: 4,
            },
        };
        let (_, monkey) = Monkey::parse(input).unwrap();
        assert_eq!(monkey, expected);
    }
    #[test]
    fn test_parse_monkeys() {
        let actual = parse_monkeys(INPUT).unwrap();
        let expected = vec![
            Monkey {
                num_inspected: 0,
                items: VecDeque::from(vec![Item { worry: 79 }, Item { worry: 98 }]),
                operation: Operation::Multiply(19),
                test: Test::DivisibleBy(23),
                test_result_monkey: TestResultTargetMonkey {
                    when_true: 2,
                    when_false: 3,
                },
                number: 0,
            },
            Monkey {
                items: VecDeque::from(vec![
                    Item { worry: 54 },
                    Item { worry: 65 },
                    Item { worry: 75 },
                    Item { worry: 74 },
                ]),
                operation: Operation::Add(6),
                test: Test::DivisibleBy(19),
                test_result_monkey: TestResultTargetMonkey {
                    when_true: 2,
                    when_false: 0,
                },
                number: 1,
                num_inspected: 0,
            },
            Monkey {
                items: VecDeque::from(vec![
                    Item { worry: 79 },
                    Item { worry: 60 },
                    Item { worry: 97 },
                ]),
                operation: Operation::Square,
                test: Test::DivisibleBy(13),
                test_result_monkey: TestResultTargetMonkey {
                    when_true: 1,
                    when_false: 3,
                },
                number: 2,
                num_inspected: 0,
            },
            Monkey {
                items: VecDeque::from(vec![Item { worry: 74 }]),
                operation: Operation::Add(3),
                test: Test::DivisibleBy(17),
                test_result_monkey: TestResultTargetMonkey {
                    when_true: 0,
                    when_false: 1,
                },
                number: 3,
                num_inspected: 0,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_simulation() {
        let actual = simulation(INPUT, NUM_ROUNDS as u64, 3);
        assert_eq!(actual, 10605);
    }
}
