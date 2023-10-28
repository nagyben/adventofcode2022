use std::{
    cmp::{max, min},
    collections::HashSet,
};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::{map_res, opt, recognize},
    multi::{many0, many1},
    sequence::preceded,
    Err, IResult, InputIter, InputLength,
};

struct Cave<'a>(&'a Vec<Vec<char>>);

fn parse(input: &str) -> Result<String, Err<nom::error::Error<&str>>> {
    let (input, sensors_beacons) = many1(parse_line)(input)?;
    let cave_bounds = get_sensor_beacon_bounds(&sensors_beacons);
    let X_OFFSET: isize = cave_bounds.min_x;
    let mut output = vec![
        vec!['.'; (cave_bounds.max_x - cave_bounds.min_x) as usize];
        cave_bounds.max_y as usize
    ];
    // for (sensor, beacon) in sensors_beacons {
    //     output[sensor.y][sensor.x] = 'S';
    //     output[beacon.y][beacon.x] = 'B';
    // }
    Ok(cave_to_string(&output))
}

fn parse_sensor_beacon(input: &str) -> Vec<(Sensor, Beacon)> {
    let (input, sensors_beacons) = many1(parse_line)(input).unwrap();
    sensors_beacons
}
#[derive(Clone, Debug)]
struct Sensor {
    x: isize,
    y: isize,
}
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Beacon {
    x: isize,
    y: isize,
}

fn parse_isize(input: &str) -> IResult<&str, isize> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        isize::from_str_radix(s, 10)
    })(input)?;

    Ok((i, number))
}
fn parse_line(input: &str) -> IResult<&str, (Sensor, Beacon)> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, sensor_x) = parse_isize(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, sensor_y) = parse_isize(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, beacon_x) = parse_isize(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, beacon_y) = parse_isize(input)?;
    Ok((
        input,
        (
            Sensor {
                x: sensor_x,
                y: sensor_y,
            },
            Beacon {
                x: beacon_x,
                y: beacon_y,
            },
        ),
    ))
}
struct CaveBounds {
    min_x: isize,
    min_y: isize,
    max_x: isize,
    max_y: isize,
}

fn get_sensor_beacon_bounds(sensors_beacons: &Vec<(Sensor, Beacon)>) -> CaveBounds {
    sensors_beacons.iter().fold(
        CaveBounds {
            min_x: isize::max_value(),
            min_y: isize::max_value(),
            max_x: isize::min_value(),
            max_y: isize::min_value(),
        },
        |bounds, (sensor, beacon)| CaveBounds {
            min_x: min(bounds.min_x, min(sensor.x, beacon.x)),
            min_y: min(bounds.min_y, min(sensor.y, beacon.y)),
            max_x: max(bounds.max_x, max(sensor.x, beacon.x)),
            max_y: max(bounds.max_y, max(sensor.y, beacon.y)),
        },
    )
}

fn cave_to_string(cave: &Vec<Vec<char>>) -> String {
    let mut output: String = "".to_string();
    cave.iter()
        .for_each(|row| output += &row.into_iter().collect::<String>());
    output
}
fn manhattan_distance(sensor: &Sensor, beacon: &Beacon) -> usize {
    ((sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs()) as usize
}
fn beaconless_positions(input: &str, target_row: usize) -> usize {
    let sb = parse_sensor_beacon(input);
    let beaconless_positions = beaconless_positions_set(&sb, target_row);
    // now we need to subtract the number of beacons that are in the target row
    let beacons_in_target_row = number_of_beacons_in_row(sb, target_row);
    beaconless_positions.len() - beacons_in_target_row
}

fn beaconless_positions_set(sb: &Vec<(Sensor, Beacon)>, target_row: usize) -> HashSet<isize> {
    let mut beaconless_positions = HashSet::new();
    sb.iter().for_each(|(sensor, beacon)| {
        let distance = manhattan_distance(sensor, beacon) as isize;
        let row_difference = (target_row as isize - sensor.y).abs();
        if distance >= row_difference {
            let leftside = sensor.x - ((distance - row_difference) as isize);
            let rightside = sensor.x + ((distance - row_difference) as isize);
            for x in leftside..=rightside {
                beaconless_positions.insert(x);
            }
        }
    });
    beaconless_positions
}

fn number_of_beacons_in_row(sb: Vec<(Sensor, Beacon)>, target_row: usize) -> usize {
    let beacons_in_target_row = sb
        .iter()
        .map(|(_, beacon)| beacon)
        .filter(|beacon| beacon.y == target_row as isize)
        .collect::<HashSet<&Beacon>>()
        .len();
    beacons_in_target_row
}

fn cave_vec(input: &str, max_size: usize) -> Vec<Vec<char>> {
    let sb = parse_sensor_beacon(input);
    let mut out = vec![vec!['.'; max_size]; max_size];

    for (sensor, beacon) in sb.iter() {
        let distance = manhattan_distance(sensor, beacon);
        let mut x = 1;
        for y in sensor.y - (distance as isize)..sensor.y {
            println!("{}", y);
            let mut _y = max(y, 0) as usize;

            let x_min = max(sensor.x - x, 0) as usize;
            let x_max = min(sensor.x + x, (max_size - 1) as isize) as usize;
            for _x in x_min..=x_max {
                out[_y][_x] = '#';
            }
            x += 2;
        }
    }
    out
}
pub fn part1() -> usize {
    let input = include_str!("input.txt");
    beaconless_positions(input, 2000000)
}
pub fn part2() -> usize {
    let input = include_str!("input.txt");
    let bp_set = beaconless_positions_set(&parse_sensor_beacon(input), 2000000);
    3
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3"#;

    #[test]
    fn test_parse() {
        let actual = parse(INPUT).unwrap();
        let expected = r#"....S.......................
......................S.....
...............S............
................SB..........
............................
............................
............................
..........S.......S.........
............................
............................
....B.......................
..S.........................
............................
............................
..............S.......S.....
B...........................
...........SB...............
................S..........B
....S.......................
............................
............S......S........
............................
.......................B...."#;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_beaconless_positions() {
        let actual = beaconless_positions(INPUT, 10);
        let expected = 26;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_manhattan_distance() {
        let sensor = Sensor { x: -2, y: 18 };
        let beacon = Beacon { x: 2, y: -15 };
        let expected = 37;
        let actual = manhattan_distance(&sensor, &beacon);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_cave_vec() {
        let actual = cave_to_string(&cave_vec(INPUT, 20));
        let expected = r#"....S.......................
......................S.....
...............S............
................SB..........
............................
............................
............................
..........S.......S.........
............................
............................
....B.......................
..S.........................
............................
............................
..............S.......S.....
B...........................
...........SB...............
................S..........B
....S.......................
............................
............S......S........
............................
.......................B...."#;
        println!("{:?}", actual);
        println!("{:?}", expected);
        assert_eq!(actual, expected);
    }
}
