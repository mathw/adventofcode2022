use crate::day::{Day, DayResult, PartResult};
use std::error::Error;

pub struct Day6 {
    input: &'static str,
}

impl Day6 {
    pub fn new() -> Day6 {
        Day6 {
            input: include_str!("inputs/day6.txt"),
        }
    }
}

impl Day for Day6 {
    fn run(&mut self) -> Result<DayResult, Box<dyn Error>> {
        let part1_result = run_part1(self.input.chars())?;
        let part2_result = run_part2(self.input.chars())?;
        Ok(DayResult::new(
            PartResult::Success(format!("The first packet begins at {}", part1_result)),
            PartResult::Success(format!("The first packet begins at {}", part2_result)),
        ))
    }
}

fn run_part1(input: impl Iterator<Item = char> + Clone) -> Result<usize, String> {
    find_packet_marker_of_size::<4>(input)
        .ok_or_else(|| "Could not find a start of packet marker".to_owned())
}

fn run_part2(input: impl Iterator<Item = char> + Clone) -> Result<usize, String> {
    find_packet_marker_of_size::<14>(input)
        .ok_or_else(|| "Could not find a start of packet marker".to_owned())
}

fn find_packet_marker_of_size<const COUNT: usize>(
    input: impl Iterator<Item = char> + Clone,
) -> Option<usize> {
    let mut last_n = ['\0'; COUNT];
    // initialise initial buffer
    for (i, c) in input.clone().take(COUNT).enumerate() {
        last_n[i] = c;
    }
    if are_all_different(&last_n) {
        return Some(COUNT);
    }
    for (i, c) in input.skip(COUNT).enumerate() {
        last_n.rotate_left(1);
        last_n[COUNT - 1] = c;
        if are_all_different(&last_n) {
            return Some(i + 1 + COUNT);
        }
    }
    None
}

fn are_all_different<const COUNT: usize>(buffer: &[char; COUNT]) -> bool {
    // probably a way to do this without allocating a new HashSet every time
    use std::collections::HashSet;
    let mut s = HashSet::new();
    for c in buffer.iter() {
        s.insert(c);
    }
    s.len() == COUNT
}

#[test]
fn test_part_one_samples() {
    assert_eq!(
        find_packet_marker_of_size::<4>("mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars()),
        Some(7)
    );
    assert_eq!(
        find_packet_marker_of_size::<4>("bvwbjplbgvbhsrlpgdmjqwftvncz".chars()),
        Some(5)
    );
    assert_eq!(
        find_packet_marker_of_size::<4>("nppdvjthqldpwncqszvftbrmjlhg".chars()),
        Some(6)
    );
    assert_eq!(
        find_packet_marker_of_size::<4>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars()),
        Some(10)
    );
    assert_eq!(
        find_packet_marker_of_size::<4>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars()),
        Some(11)
    );
}

#[test]
fn test_part_two_samples() {
    assert_eq!(
        find_packet_marker_of_size::<14>("mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars()),
        Some(19)
    );
    assert_eq!(
        find_packet_marker_of_size::<14>("bvwbjplbgvbhsrlpgdmjqwftvncz".chars()),
        Some(23)
    );
    assert_eq!(
        find_packet_marker_of_size::<14>("nppdvjthqldpwncqszvftbrmjlhg".chars()),
        Some(23)
    );
    assert_eq!(
        find_packet_marker_of_size::<14>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars()),
        Some(29)
    );
    assert_eq!(
        find_packet_marker_of_size::<14>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars()),
        Some(26)
    );
}
