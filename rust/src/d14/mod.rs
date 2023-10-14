use std::error::Error;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map_res,
    multi::{many0, separated_list1},
    sequence::separated_pair,
    IResult,
};
const SAND_ENTRY_X: usize = 500;
type CaveMap = Vec<Vec<char>>;
trait Parseable {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}
fn parse_cave_polygons(input: &str) -> Result<Vec<Polygon>, nom::Err<nom::error::Error<&str>>> {
    let (_, polygons) = many0(Polygon::parse)(input)?;
    Ok(polygons)
}

fn get_cave_bounds(polygons: &Vec<Polygon>) -> (usize, usize, usize) {
    polygons.iter().fold(
        (SAND_ENTRY_X, SAND_ENTRY_X, 0),
        |(min, max, height), polygon| {
            polygon
                .points
                .iter()
                .fold((min, max, height), |(min, max, height), point| {
                    let new_min = std::cmp::min(point.x, min);
                    let new_max = std::cmp::max(point.x, max);
                    let new_height = std::cmp::max(point.y, height);
                    (new_min, new_max, new_height)
                })
        },
    )
}

fn print_cave(cave: &CaveMap) {
    for row in cave {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Parseable for Point {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, coords) = separated_pair(
            map_res(digit1, str::parse),
            tag(","),
            map_res(digit1, str::parse),
        )(input)?;
        let x = coords.0;
        let y = coords.1;
        Ok((input, Point { x, y }))
    }
}
#[derive(Debug, PartialEq)]
struct Polygon {
    points: Vec<Point>,
}

impl Parseable for Polygon {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = multispace0(input)?;
        let (input, points) = separated_list1(tag(" -> "), Point::parse)(input)?;
        Ok((input, Polygon { points }))
    }
}
fn parse_cave(polygons: &Vec<Polygon>) -> Vec<Vec<char>> {
    let (min, max, height) = get_cave_bounds(&polygons);
    let mut cave = vec![vec!['.'; max - min + 1]; height + 1];
    for polygon in polygons {
        for (i, point) in polygon.points.iter().enumerate() {
            if let Some(next_point) = polygon.points.get(i + 1) {
                if point.y == next_point.y {
                    for x in
                        std::cmp::min(point.x, next_point.x)..=std::cmp::max(point.x, next_point.x)
                    {
                        cave[point.y][x - min] = '#';
                    }
                } else {
                    for y in
                        std::cmp::min(point.y, next_point.y)..=std::cmp::max(point.y, next_point.y)
                    {
                        cave[y][point.x - min] = '#';
                    }
                }
            }
        }
    }
    cave[0][SAND_ENTRY_X - min] = '+';
    cave
}

struct Sand {
    position: Point,
}

enum FallDirection {
    Left,
    Right,
    Down,
}
#[derive(Debug)]
enum FallError {
    OutOfBounds,
}
#[derive(Debug, PartialEq)]
enum SandState {
    Falling,
    Resting,
}
impl Sand {
    fn fall(&mut self, cave: &CaveMap) -> Result<SandState, FallError> {
        if self.try_fall(cave, FallDirection::Down)?
            || self.try_fall(cave, FallDirection::Left)?
            || self.try_fall(cave, FallDirection::Right)?
        {
            return Ok(SandState::Falling);
        }
        Ok(SandState::Resting)
    }
    fn try_fall(&mut self, cave: &CaveMap, direction: FallDirection) -> Result<bool, FallError> {
        let mut next_position = self.position.clone();

        // we are always checking one row below
        next_position.y = self.position.y + 1;

        // check if we are trying to fall out of bounds
        if next_position.y > cave.len() - 1 {
            return Err(FallError::OutOfBounds.into());
        }

        match direction {
            FallDirection::Left => {
                if self.position.x == 0 {
                    // we've fallen outside the bounds of the cave
                    // if this happens we assume the caller has already tried another direction
                    // so we return an error
                    return Err(FallError::OutOfBounds.into());
                }
                next_position.x = self.position.x - 1;

                // only move if the next position is empty
                if cave[next_position.y][next_position.x] == '.' {
                    self.position = next_position;
                    return Ok(true);
                }

                // otherwise is no bueno
                return Ok(false);
            }
            FallDirection::Right => {
                next_position.x = self.position.x + 1;
                if next_position.x > cave[0].len() - 1 {
                    // we've fallen outside the bounds of the cave
                    // if this happens we assume the caller has already tried another direction
                    // so we return an error
                    return Err(FallError::OutOfBounds.into());
                }

                // only move if the next position is empty
                if cave[next_position.y][next_position.x] == '.' {
                    self.position = next_position;
                    return Ok(true);
                }

                // otherwise is no bueno
                return Ok(false);
            }
            FallDirection::Down => {
                // only move if the next position is empty
                if cave[next_position.y][next_position.x] == '.' {
                    self.position = next_position;
                    return Ok(true);
                }

                // otherwise is no bueno
                return Ok(false);
            }
        };
    }
}
fn fall_sand(input: &str, bedrock: bool) -> CaveMap {
    let mut polygons = parse_cave_polygons(input).unwrap();
    let (mut min, _, mut height) = get_cave_bounds(&polygons);
    if bedrock {
        polygons.push(Polygon {
            points: vec![
                Point {
                    x: SAND_ENTRY_X - height - 2,
                    y: height + 2,
                },
                Point {
                    x: SAND_ENTRY_X + height + 2,
                    y: height + 2,
                },
            ],
        });
        (min, _, height) = get_cave_bounds(&polygons);
    }
    let mut cave = parse_cave(&polygons);
    let sand_entry: usize = SAND_ENTRY_X - min;

    'outer: loop {
        // if the source is blocked, we are done
        if cave[0][sand_entry] != '+' {
            break 'outer;
        }

        let mut sand = Sand {
            position: Point {
                x: sand_entry,
                y: 0,
            },
        };

        // loop this until we are no longer falling
        const MAX_ITERATIONS: usize = 10000;
        let mut iterations = 0;
        'inner: loop {
            match sand.fall(&cave) {
                Ok(SandState::Falling) => {}
                Ok(SandState::Resting) => break 'inner,
                Err(FallError::OutOfBounds) => break 'outer,
            }

            iterations += 1;
            if iterations > MAX_ITERATIONS {
                panic!("too many iterations");
            }
        }
        cave[sand.position.y][sand.position.x] = 'o';
    }
    cave
}

fn count_sand(cave: &CaveMap) -> usize {
    cave.iter()
        .map(|row| row.iter().filter(|col| **col == 'o').count())
        .sum()
}
pub fn part1(input: &str) -> usize {
    let cave = fall_sand(input, false);
    count_sand(&cave)
}

pub fn part2(input: &str) -> usize {
    let cave = fall_sand(input, true);
    count_sand(&cave)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cave() {
        const INPUT: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

        let expected: CaveMap = vec![
            vec!['.', '.', '.', '.', '.', '.', '+', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '#', '#'],
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '#', '.'],
            vec!['.', '.', '#', '#', '#', '.', '.', '.', '#', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '.'],
        ];
        let actual = parse_cave(&parse_cave_polygons(INPUT).unwrap());
        print_cave(&actual);
        print_cave(&expected);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_get_cave_bounds() {
        const INPUT: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

        let expected = (494, 503, 9);
        let actual = get_cave_bounds(&parse_cave_polygons(INPUT).unwrap());
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_parse_cave_polygons() {
        const INPUT: &str = r#"498,4 -> 498,6 -> 496,6"#;
        let expected = vec![Polygon {
            points: vec![
                Point { x: 498, y: 4 },
                Point { x: 498, y: 6 },
                Point { x: 496, y: 6 },
            ],
        }];
        let actual = parse_cave_polygons(INPUT).unwrap();
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_fall_sand() {
        const INPUT: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;
        let expected: CaveMap = vec![
            vec!['.', '.', '.', '.', '.', '.', '+', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', 'o', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', 'o', 'o', 'o', '.', '.'],
            vec!['.', '.', '.', '.', '#', 'o', 'o', 'o', '#', '#'],
            vec!['.', '.', '.', 'o', '#', 'o', 'o', 'o', '#', '.'],
            vec!['.', '.', '#', '#', '#', 'o', 'o', 'o', '#', '.'],
            vec!['.', '.', '.', '.', 'o', 'o', 'o', 'o', '#', '.'],
            vec!['.', 'o', '.', 'o', 'o', 'o', 'o', 'o', '#', '.'],
            vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '.'],
        ];
        let actual = fall_sand(INPUT, false);
        print_cave(&actual);
        print_cave(&expected);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_fall_sand_2() {
        const INPUT: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

        let expected: CaveMap = vec![
            vec!['.', '.', '.', '.', '.', '.', '+', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', 'o', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', 'o', 'o', 'o', '.', '.'],
            vec!['.', '.', '.', '.', '#', 'o', 'o', 'o', '#', '#'],
            vec!['.', '.', '.', 'o', '#', 'o', 'o', 'o', '#', '.'],
            vec!['.', '.', '#', '#', '#', 'o', 'o', 'o', '#', '.'],
            vec!['.', '.', '.', '.', 'o', 'o', 'o', 'o', '#', '.'],
            vec!['.', 'o', '.', 'o', 'o', 'o', 'o', 'o', '#', '.'],
            vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '.'],
        ];
        let actual = fall_sand(INPUT, true);
        print_cave(&actual);
        print_cave(&expected);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_count_sand() {
        const INPUT: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;
        let cave = fall_sand(INPUT, false);
        let expected = 24;
        let actual = count_sand(&cave);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_count_sand2() {
        const INPUT: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;
        let cave = fall_sand(INPUT, true);
        let expected = 93;
        let actual = count_sand(&cave);
        assert_eq!(expected, actual);
    }
}
