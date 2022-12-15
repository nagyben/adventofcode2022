use std::collections::HashSet;

fn program(input: &str) -> Vec<i32> {
    let mut x: Vec<i32> = vec![1];

    for (i, line) in input.split("\n").collect::<Vec<&str>>().iter().enumerate() {
        let prev = x.last().unwrap().to_owned();
        if &line[0..4] == "addx" {
            x.push(prev);
            let arg: i32 = line[5..].parse::<i32>().unwrap();
            x.push(prev + arg);
        }

        if &line[0..4] == "noop" {
            x.push(prev);
        }
    }
    x
}

fn draw_program(program: &Vec<i32>) {
    for j in 0..6 {
        let mut row: String = String::from("");
        for i in 0..40 {
            if program[i + j*40] >= i as i32 - 1
            && program[i + j*40] <= i as i32 + 1 {
                row.push_str("@");
            } else {
                row.push_str(" ");
            }
        }
        println!("{}", row);
    }
}

pub fn total_signal_strength(input: &str) -> i32 {
    let x = program(input);
    let cycles_of_interest: HashSet<usize> = HashSet::from([20, 60, 100, 140, 180, 220]);
    let mut sum = 0;
    for k in cycles_of_interest {
        sum += x[k-1] * k as i32;
    }
    sum
}

pub fn draw_crt(input: &str) {
    draw_program(&program(input));
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_history() {
        let input = r#"noop
addx 3
addx -5"#;

        let actual = program(input);
        let expected = vec![
            1, // start
            1, // noop
            1, // addx 3
            4, // addx 3
            4, // addx -5
            -1 // addx -5
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_cycles() {
        let input = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;
        let actual = program(input);
        let expected: HashMap<usize, i32> = HashMap::from([
            (20, 21),
            (60, 19),
            (100, 18),
            (140, 21),
            (180, 16),
            (220, 18),
        ]);

        for (k, v) in expected {
            // k is after the cycle but we want during the cycle
            // so use k-1
            assert_eq!(actual[k-1], v, "expected index {} to be {}", k, v);
        }
    }

    #[test]
    fn test_total_signal_strength() {
        let input = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;
        assert_eq!(total_signal_strength(input), 13140);

    }

    #[test]
    fn test_draw_program() {
        let input = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

        draw_program(&program(input));
        assert!(false);
    }
}