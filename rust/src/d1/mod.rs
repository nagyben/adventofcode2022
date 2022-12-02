use std::fs;

pub fn get_calories_by_elf(input: String) -> Vec<u32> {
    let calories: Vec<u32> = input
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<u32>().unwrap_or_default())
        .collect();

    let mut calories_by_elf: Vec<u32> = vec![];
    let mut cur_max: u32 = 0;
    let num_snacks = &calories.len();
    for (i, calorie) in calories.iter().enumerate() {
        if *calorie == 0 {
            calories_by_elf.push(cur_max);
            cur_max = 0;
        } else if i == *num_snacks - 1 {
            cur_max += calorie;
            calories_by_elf.push(cur_max);
        } else {
            cur_max += calorie
        }
    }

    calories_by_elf
}

pub fn get_most_calories(input: String) -> u32 {
    let mut calories_by_elf = get_calories_by_elf(input);
    calories_by_elf.sort();
    calories_by_elf.reverse();
    calories_by_elf[0]
}

pub fn get_top3_calories(input: String) -> u32 {
    let mut calories_by_elf = get_calories_by_elf(input);
    calories_by_elf.sort();
    calories_by_elf.reverse();
    calories_by_elf[0..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_calories_by_elf() {
        let test_contents = fs::read_to_string("src/d1/test_input.txt").unwrap();
        let expected = vec![6000, 4000, 11000, 24000, 10000];
        let actual = get_calories_by_elf(test_contents);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_one() {
        let test_contents = fs::read_to_string("src/d1/test_input.txt").unwrap();
        let expected = 24000;
        let actual = get_most_calories(test_contents);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let test_contents = fs::read_to_string("src/d1/test_input.txt").unwrap();
        let expected = 45000;
        let actual = get_top3_calories(test_contents);

        assert_eq!(expected, actual);
    }
}
