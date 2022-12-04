use std::collections::HashSet;

fn get_shared_items_in_rucksack_compartments(input: &str) -> Vec<char> {
    // split input 50/50
    let num_items_per_compartment = input.len() / 2;
    let compartment1 = input[..num_items_per_compartment]
        .chars()
        .into_iter()
        .collect::<HashSet<char>>();
    let compartment2 = input[num_items_per_compartment..]
        .chars()
        .into_iter()
        .collect::<HashSet<char>>();

    compartment1
        .into_iter()
        .filter(|e| compartment2.contains(e))
        .collect()
}

fn get_item_priority(input: char) -> u32 {
    if input as u32 >= 97 {
        // lowercase a-z starts at ascii 97
        // the priority for a-z is 1-26
        input as u32 - 96
    } else {
        // uppercase A-Z starts at ascii 65
        // the priority for A-Z is 27-52
        input as u32 + 26 - 64
    }
}

pub fn get_priority_sum(input: &str) -> u32 {
    let shared_items: Vec<Vec<char>> = input
    .split("\n")
    .map(|x| get_shared_items_in_rucksack_compartments(x))
    .collect();
    let priorities: Vec<Vec<u32>> = shared_items
        .into_iter()
        .map(|x| x.into_iter().map(get_item_priority).collect())
        .collect();

    priorities.into_iter().flatten().collect::<Vec<u32>>().iter().sum()
}

pub fn get_group_badge(input: &str) -> char {
    let mut unique_items_per_elf = input.split("\n").collect::<Vec<&str>>().iter().map(|x| x.chars().into_iter()
    .collect::<HashSet<char>>()).collect::<Vec<HashSet<char>>>();

    let mut result = unique_items_per_elf.pop().unwrap();
    result.retain(|item| {
        unique_items_per_elf.iter().all(|set| set.contains(item))
    });
    assert_eq!(result.len(), 1);
    result.into_iter().collect::<Vec<char>>()[0]
}

fn get_groups(input: &str) -> Vec<String> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    assert_eq!(lines.len() % 3, 0);
    let mut result: Vec<String> = vec![];
    let mut i = 0;
    while i < lines.len() - 1 {
        result.push(lines[i..i+3].join("\n"));
        i += 3;
    }
    result
}

pub fn get_badge_priority_total(input: &str) -> u32 {
    let groups = get_groups(input);
    let badges: Vec<char> = groups.into_iter().map(|x| get_group_badge(x.as_str())).collect();
    badges.into_iter().map(get_item_priority).collect::<Vec<u32>>().iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("vJrwpWtwJgWrhcsFMMfFFhFp", vec!['p'])]
    #[case("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", vec!['L'])]
    #[case("PmmdzqPrVvPwwTWBwg", vec!['P'])]
    #[case("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", vec!['v'])]
    #[case("ttgJtRGJQctTZtZT", vec!['t'])]
    #[case("CrZsJsPPZsGzwwsLwLmpwMDw", vec!['s'])]
    fn test_shared_items_in_rucksack_compartments(#[case] input: &str, #[case] expected: Vec<char>) {
        let actual = get_shared_items_in_rucksack_compartments(input);
        println!("{:?}", actual);
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case('a', 1)]
    #[case('b', 2)]
    #[case('l', 12)]
    #[case('p', 16)]
    #[case('s', 19)]
    #[case('t', 20)]
    #[case('v', 22)]
    #[case('z', 26)]
    #[case('L', 38)]
    #[case('P', 42)]
    fn test_get_item_priority(#[case] input: char, #[case] expected: u32) {
        assert_eq!(get_item_priority(input), expected, "expected {} for {}", expected, input);
    }

    #[test]
    fn test_get_priority_sum() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

        let actual = get_priority_sum(input);
        let expected = 157;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_group_badge() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg"#;
        let expected = 'r';
        let actual = get_group_badge(input);
        assert_eq!(actual, expected);

        let input = r#"wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        let expected = 'Z';
        let actual = get_group_badge(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_groups() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        let expected: Vec<&str> = vec!["vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg", "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw"];
        let actual = get_groups(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_groups_badge_priority_total() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

        let expected = 70;
        let actual = get_badge_priority_total(input);
        assert_eq!(actual, expected);
    }
}
