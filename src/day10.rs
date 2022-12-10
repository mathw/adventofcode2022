use std::error::Error;

use crate::common::cpu::Cpu;
use crate::common::display::Display;
use crate::day::{Day, DayResult, PartResult};

pub struct Day10 {
    input: &'static str,
}

impl Day10 {
    pub fn new() -> Day10 {
        Day10 {
            input: include_str!("inputs/day10.txt"),
        }
    }
}

impl Day for Day10 {
    fn run(&mut self) -> Result<DayResult, Box<dyn Error>> {
        let part1 = run_part1(self.input)?;
        let part2 = run_part2(self.input)?;
        Ok(DayResult::new(
            PartResult::Success(format!("Signal strength is {}", part1)),
            PartResult::Success(format!("\n{}", part2)),
        ))
    }
}

fn run_part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut cpu = Cpu::compile(input)?;
    let mut result = 0;
    let x = run_cycles(&mut cpu, 20); // 20
    result += x * 20;
    let x = run_cycles(&mut cpu, 40); // 60
    result += x * 60;
    let x = run_cycles(&mut cpu, 40); // 100
    result += x * 100;
    let x = run_cycles(&mut cpu, 40); // 140
    result += x * 140;
    let x = run_cycles(&mut cpu, 40); // 180
    result += x * 180;
    let x = run_cycles(&mut cpu, 40); // 220
    result += x * 220;
    Ok(result)
}

fn run_cycles(cpu: &mut Cpu, cycles: u32) -> i32 {
    let mut x = cpu.get_x();
    for _ in 0..cycles {
        x = cpu.cycle();
    }
    x
}

fn run_part2(input: &str) -> Result<String, Box<dyn Error>> {
    let mut cpu = Cpu::compile(input)?;
    let mut display = Display::new(40, 6);
    let values = cpu.run_to_completion();
    for (beam_position, value) in values.into_iter().enumerate() {
        let x = beam_position % 40;
        let y = beam_position / 40;
        if y >= 6 || x >= 40 {
            println!("Terminating early");
            break;
        }
        let pixel = x as i32 == value || x as i32 + 1 == value || x as i32 == value + 1;
        display.set(x, y, pixel)?;
        println!("Setting {},{} to {}", x, y, pixel);
    }
    Ok(display.to_string())
}

#[test]
fn test_part1_sample() {
    let input = include_str!("inputs/day10-sample.txt");
    let result = run_part1(input).expect("This should compile really");
    assert_eq!(result, 13140);
}

#[test]
fn test_part1_sample_stages() {
    let input = include_str!("inputs/day10-sample.txt");
    let mut cpu = Cpu::compile(input).expect("This should compile");
    let x = run_cycles(&mut cpu, 20); // 20
    let strength = x * 20;
    assert_eq!(strength, 420);
    let x = run_cycles(&mut cpu, 40); // 60
    let strength = x * 60;
    assert_eq!(strength, 1140);
    let x = run_cycles(&mut cpu, 40); // 100
    let strength = x * 100;
    assert_eq!(strength, 1800);
    let x = run_cycles(&mut cpu, 40); // 140
    let strength = x * 140;
    assert_eq!(strength, 2940);
    let x = run_cycles(&mut cpu, 40); // 180
    let strength = x * 180;
    assert_eq!(strength, 2880);
    let x = run_cycles(&mut cpu, 40); // 220
    let strength = x * 220;
    assert_eq!(strength, 3960);
}

#[test]
fn test_part2_sample() {
    let input = include_str!("inputs/day10-sample.txt");
    let output = run_part2(input).expect("Should not explode");
    let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
    assert_eq!(&output, expected);
}
