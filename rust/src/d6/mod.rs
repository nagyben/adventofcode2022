use std::collections::HashSet;

pub fn find_marker(input: &str, num_distint_chars: usize) -> usize {
    for i in 0..input.len() {
        let hs: HashSet<char> = HashSet::from_iter(input[i..i+num_distint_chars].chars());
        if hs.len() == num_distint_chars {
            return i+num_distint_chars
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn test_find_marker_part1(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(find_marker(input, 4), expected);
    }

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn test_find_marker_part2(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(find_marker(input, 14), expected);
    }
}