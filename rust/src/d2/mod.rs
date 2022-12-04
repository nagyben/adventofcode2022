pub fn part1_map(hand: &str) -> u32 {
    match hand {
        "A X" => 1 + 3, //rock-rock draw
        "A Y" => 2 + 6, //rock-paper win
        "A Z" => 3 + 0, //rock-scissor lose

        "B X" => 1 + 0, //paper-rock lose
        "B Y" => 2 + 3, //paper-paper draw
        "B Z" => 3 + 6, //paper-scissor win

        "C X" => 1 + 6, //scissor-rock win
        "C Y" => 2 + 0, //scissor-paper lose
        "C Z" => 3 + 3, //scissor-scissor draw
        &_ => 0,
    }
}

pub fn strategy(input: &str, mapper: &dyn Fn(&str) -> u32) -> u32 {
    let games: Vec<u32> = input.split("\n").map(|x| mapper(x)).collect();
    games.iter().sum()
}

pub fn part2_map(hand: &str) -> u32 {
    match hand {
        "A X" => 3 + 0, //lose: rock-scissors
        "A Y" => 1 + 3, //draw: rock-rock
        "A Z" => 2 + 6, //win:  rock-paper

        "B X" => 1 + 0, //lose: paper-rock
        "B Y" => 2 + 3, //draw: paper-paper
        "B Z" => 3 + 6, //win:  paper-scissor

        "C X" => 2 + 0, //lose: scissor-paper
        "C Y" => 3 + 3, //draw: scissor-scissor
        "C Z" => 1 + 6, //win:  scissor-rock
        &_ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("A X", 1 + 3)] //rock-rock draw
    #[case("A Y", 2 + 6)] //rock-paper win
    #[case("A Z", 3 + 0)] //rock-scissor lose
    #[case("B X", 1 + 0)] //paper-rock lose
    #[case("B Y", 2 + 3)] //paper-paper draw
    #[case("B Z", 3 + 6)] //paper-scissor win
    #[case("C X", 1 + 6)] //scissor-rock win
    #[case("C Y", 2 + 0)] //scissor-paper lose
    #[case("C Z", 3 + 3)] //scissor-scissor draw
    fn test_part1_map(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(expected, part1_map(input));
    }

    #[test]
    fn test_part1() {
        let input = r#"A Y
B X
C Z"#;

        let actual = strategy(input, &part1_map);
        let expected = 15;

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("A X", 3 + 0)] //lose: rock-scissors
    #[case("A Y", 1 + 3)] //draw: rock-rock
    #[case("A Z", 2 + 6)] //win:  rock-paper
    #[case("B X", 1 + 0)] //lose: paper-rock
    #[case("B Y", 2 + 3)] //draw: paper-paper
    #[case("B Z", 3 + 6)] //win:  paper-scissor
    #[case("C X", 2 + 0)] //lose: scissor-paper
    #[case("C Y", 3 + 3)] //draw: scissor-scissor
    #[case("C Z", 1 + 6)] //win:  scissor-rock
    fn test_part2_map(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(expected, part2_map(input));
    }

    #[test]
    fn test_part2() {
        let input = r#"A Y
B X
C Z"#;

        let actual = strategy(input, &part2_map);
        let expected = 12;

        assert_eq!(actual, expected);
    }
}
