use std::{collections::HashSet, hash::Hash, rc::Rc};

#[derive(PartialEq, Debug, Default, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from_tuple(xy: (i32, i32)) -> Point {
        Point { x: xy.0, y: xy.1 }
    }
    fn motion(&mut self, input: &str) {
        let m: Vec<&str> = input.split(" ").collect::<Vec<&str>>();
        let size: i32 = m[1].parse::<i32>().unwrap();
        match m[0] {
            "R" => self.x += size,
            "L" => self.x -= size,
            "U" => self.y += size,
            "D" => self.y -= size,
            x => {
                panic!("Got {:?} for move instruction", x)
            },
        }
    }

    fn do_move(&mut self, input: &str) {
        match input {
            "R" => self.x += 1,
            "L" => self.x -= 1,
            "U" => self.y += 1,
            "D" => self.y -= 1,
            x => {
                panic!("Got {:?} for move instruction", x)
            },
        }
    }

    fn follow(&mut self, leader: &Point) {
        let dx = leader.x - self.x;
        let dy = leader.y - self.y;
        if dx.abs() + dy.abs() > 2 {
            // diagonal move required
            // diagonal move is a (1,1) move in the dx/dy direction
            self.x += if dx > 0 { 1 } else { -1 };
            self.y += if dy > 0 { 1 } else { -1 };

        } else if dx.abs() > 1 {
            self.x += if dx > 0 { 1 } else { -1 };

        } else if dy.abs() > 1 {
            self.y += if dy > 0 { 1 } else { -1 };
        }
    }

    fn get_xy(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

fn short_rope(input: &str) -> HashSet<(i32, i32)> {
    let mut head = Point::from_tuple((0,0));
    let mut tail = Point::from_tuple((0,0));
    let mut visited: HashSet<(i32, i32)> = vec![].into_iter().collect();

    for line in input.split("\n").collect::<Vec<&str>>() {
        let m = line.split(" ").collect::<Vec<&str>>();
        let move_size = m[1].parse().unwrap();
        for _ in 0..move_size {
            head.do_move(m[0]);
            tail.follow(&head);
            visited.insert(tail.get_xy());
        }
    }
    visited
}

fn long_rope(input: &str) -> HashSet<(i32, i32)> {
    let knots = 10;
    let mut rope: Vec<Point> = vec![];
    for _ in 0..knots {
        rope.push(Point::from_tuple((0,0)));
    }

    let mut visited: HashSet<(i32, i32)> = vec![].into_iter().collect();

    for line in input.split("\n").collect::<Vec<&str>>() {
        let m = line.split(" ").collect::<Vec<&str>>();
        let move_size = m[1].parse().unwrap();
        for _ in 0..move_size {
            rope[0].do_move(m[0]);
            for i in 1..rope.len() {
                let prev_rope = Rc::new(rope[i-1]);
                rope[i].follow(&prev_rope);
            }
            visited.insert(rope.last().unwrap().get_xy());
        }
    }
    visited
}

pub fn short_rope_unique_locations(input: &str) -> usize {
    let hs = short_rope(input);
    hs.len()
}

pub fn long_rope_unique_locations(input: &str) -> usize {
    let hs = long_rope(input);
    hs.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("R", Point{x:1, y:0})]
    #[case("U", Point{x:0, y:1})]
    #[case("L", Point{x:-1, y:0})]
    #[case("D", Point{x:0, y:-1})]
    fn test_move_head(#[case] input: &str, #[case] expected: Point) {
        let mut head = Point { x: 0, y: 0 };
        head.do_move(input);
        assert_eq!(head, expected);
    }

    /* STRAIGHT MOVES */
    #[rstest]
    #[case((2,0), (1,0))]
    #[case((-2,0), (-1,0))]
    #[case((0,2), (0,1))]
    #[case((0,-2), (0,-1))]
    /* DIAGONAL MOVES */
    #[case((2,1), (1,1))]
    #[case((1,2), (1,1))]
    #[case((-1,2), (-1,1))]
    #[case((-2,1), (-1,1))]
    #[case((-2,-1), (-1,-1))]
    #[case((-1,-2), (-1,-1))]
    #[case((1,-2), (1,-1))]
    #[case((2,-1), (1,-1))]
    fn test_follow(#[case] head_location: (i32, i32), #[case] tail_final: (i32, i32)) {
        let head = Point::from_tuple(head_location);

        let mut tail = Point::from_tuple((0, 0));
        let expected = Point::from_tuple(tail_final);
        tail.follow(&head);
        assert_eq!(tail, expected);
    }

    #[test]
    fn test_run_short_rope_scenario() {
        let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

        let actual = short_rope(&input);
        let expected: HashSet<(i32, i32)> = vec![
            (0,0),
            (1,0),
            (2,0),
            (3,0),
            (4,1),
            (1,2),
            (2,2),
            (3,2),
            (4,2),
            (3,3),
            (4,3),
            (3,4),
            (2,4)
        ].into_iter().collect();
        assert_eq!(actual, expected, "difference: {:?}", actual.difference(&expected));
    }

    #[test]
    fn test_scenario_unique_locations() {
        let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
        let expected = 13;
        let actual = short_rope_unique_locations(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_long_rope_scenario() {
        let input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;
        let expected = 36;
        let actual = long_rope_unique_locations(&input);
        assert_eq!(actual, expected);
    }
}
