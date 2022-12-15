use std::{collections::HashSet, error::Error};

use crate::common::day;

pub struct Day15 {
    input: &'static str,
}

impl Day15 {
    pub fn new() -> Self {
        Self {
            input: include_str!("inputs/day15.txt"),
        }
    }
}

impl day::Day for Day15 {
    fn run(&mut self) -> day::Result {
        let sensors = parse_input(self.input)?;
        let count = count_positions_at_y_where_no_beacons_can_be_present(sensors.iter(), 2000000);
        Ok((Some(count.to_string()), None))
    }
}

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance_from(&self, other: &Position) -> u32 {
        absolute_distance(self.x, other.x) + absolute_distance(self.y, other.y)
    }
}

struct Sensor {
    position: Position,
    beacon: Position,
    beacon_range: u32,
}

impl Sensor {
    fn new(position: Position, beacon: Position) -> Self {
        let beacon_range = position.distance_from(&beacon);
        Self {
            position,
            beacon,
            beacon_range,
        }
    }

    fn can_confirm_no_beacons_at_y(&self, y: i32) -> Option<Range> {
        let neutral_position = Position::new(self.position.x, y);
        let neutral_distance = self.position.distance_from(&neutral_position);
        if neutral_distance > self.beacon_range {
            return None;
        }

        let available_range = (self.beacon_range - neutral_distance) as i32;

        let min_x_in_range = neutral_position.x - available_range;
        let max_x_in_range = neutral_position.x + available_range;

        Some(Range::new(min_x_in_range, max_x_in_range))
    }
}

#[derive(Clone)]
struct Range {
    lower: i32,
    upper: i32,
}

impl Range {
    fn new(lower: i32, upper: i32) -> Self {
        Range { lower, upper }
    }

    fn combine_with(&self, other: &Range) -> Option<Range> {
        if self.lower <= other.lower && self.upper >= other.upper {
            // other is totally contained within self
            Some(self.clone())
        } else if self.lower <= other.lower && self.upper >= other.lower {
            // overlapping - we are lower
            Some(Range::new(self.lower, other.upper))
        } else if self.lower >= other.lower && self.upper >= other.upper {
            // overlapping - we are higher
            Some(Range::new(other.lower, self.upper))
        } else if self.lower >= other.lower && self.upper <= other.upper {
            // self is totally contained within other
            Some(other.clone())
        } else {
            // no overlap, cannot combine
            None
        }
    }

    fn len(&self) -> usize {
        (0 - self.lower).unsigned_abs() as usize + self.upper.unsigned_abs() as usize
    }
}

fn absolute_distance(a: i32, b: i32) -> u32 {
    match (a > 0, b > 0) {
        (true, true) => (a - b).unsigned_abs(),
        (false, true) => (b + a.abs()) as u32,
        (true, false) => (a + b.abs()) as u32,
        (false, false) => (a - b).unsigned_abs(),
    }
}

fn parse_input_line(line: &str) -> Result<Sensor, Box<dyn Error>> {
    use regex::Regex;
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
        )
        .unwrap();
    }

    let captures = RE
        .captures(line)
        .ok_or_else(|| format!("Input string \"{}\" did not match regex", line))?;

    Ok(Sensor::new(
        Position::new(captures[1].parse()?, captures[2].parse()?),
        Position::new(captures[3].parse()?, captures[4].parse()?),
    ))
}

fn parse_input(input: &str) -> Result<Vec<Sensor>, Box<dyn Error>> {
    input.lines().map(|l| parse_input_line(l.trim())).collect()
}

fn count_positions_at_y_where_no_beacons_can_be_present<'a>(
    sensors: impl Iterator<Item = &'a Sensor>,
    y: i32,
) -> usize {
    let mut ranges: Vec<Range> = Vec::new();
    let mut beacon_positions = HashSet::new();
    for sensor in sensors {
        if sensor.beacon.y == y {
            beacon_positions.insert(sensor.beacon.x);
        }
        if let Some(range) = sensor.can_confirm_no_beacons_at_y(y) {
            let mut combined_range = false;
            for existing_range in ranges.iter_mut() {
                if let Some(new_range) = existing_range.combine_with(&range) {
                    *existing_range = new_range;
                    combined_range = true;
                    break;
                }
            }
            if !combined_range {
                ranges.push(range);
            }
        }
    }
    ranges = normalise_ranges(&ranges);
    ranges.into_iter().map(|r| r.len()).sum()
}

fn normalise_ranges(ranges: &Vec<Range>) -> Vec<Range> {
    if ranges.len() <= 1 {
        return ranges.clone();
    }

    fn normalise_pass(range: Range, others: Vec<Range>) -> (Vec<Range>, bool) {
        if others.is_empty() {
            return (vec![range], false);
        }

        let mut ranges = Vec::new();
        let mut did_combine = false;
        for other in others {
            if let Some(combined) = range.combine_with(&other) {
                ranges.push(combined);
                did_combine = true;
            } else {
                ranges.push(other);
            }
        }
        if !did_combine {
            ranges.push(range)
        }
        (ranges, did_combine)
    }

    let mut working_ranges = ranges.clone();
    working_ranges.sort_by_key(|r| r.lower);
    loop {
        if working_ranges.len() <= 1 {
            return working_ranges;
        }

        let this_range = working_ranges
            .drain(..1)
            .next()
            .expect("We already checked there was at least one range in here");

        let (new_ranges, did_combine) = normalise_pass(this_range, working_ranges);
        if !did_combine {
            return new_ranges;
        }
        working_ranges = new_ranges;
    }
}

#[test]
fn test_part1_sample() {
    let input = include_str!("inputs/day15-sample.txt");
    let sensors = parse_input(input).expect("Input should be good");
    let count = count_positions_at_y_where_no_beacons_can_be_present(sensors.iter(), 10);
    assert_eq!(count, 26);
}
