use std::cmp::max;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map_res,
    multi::{many0, many1},
    Err, IResult, InputIter, InputLength,
};

fn parse(input: &str) -> Result<String, Err<nom::error::Error<&str>>> {
    let (input, sensors_beacons) = many1(parse_line)(input)?;
    let (x, y) = get_sensor_beacon_bounds(&sensors_beacons);
    let mut output = vec![vec!['.'; x]; y];
    for (sensor, beacon) in sensors_beacons {
        output[sensor.y][sensor.x] = 'S';
        output[beacon.y][beacon.x] = 'B';
    }
    Ok(cave_to_string(&output))
}

struct Sensor {
    x: usize,
    y: usize,
}
struct Beacon {
    x: usize,
    y: usize,
}
fn parse_line(input: &str) -> IResult<&str, (Sensor, Beacon)> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, sensor_x) = map_res(digit1, str::parse)(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, sensor_y) = map_res(digit1, str::parse)(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, beacon_x) = map_res(digit1, str::parse)(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, beacon_y) = map_res(digit1, str::parse)(input)?;
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
fn get_sensor_beacon_bounds(sensors_beacons: &Vec<(Sensor, Beacon)>) -> (usize, usize) {
    sensors_beacons
        .iter()
        .fold((0, 0), |(max_x, max_y), (sensor, beacon)| {
            (
                max(max_x, max(sensor.x, beacon.x)),
                max(max_y, max(sensor.y, beacon.y)),
            )
        })
}

fn cave_to_string(cave: &Vec<Vec<char>>) -> String {
    let mut output: String = "".to_string();
    cave.iter()
        .for_each(|row| output += &row.into_iter().collect::<String>());
    output
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
}
