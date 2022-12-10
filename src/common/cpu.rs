use std::{error::Error, str::FromStr};

#[derive(Debug)]
pub struct Cpu {
    code: Vec<Instruction>,
    pc: usize,
    cycle: usize,
    x: i32,
    state: State,
}

impl Cpu {
    pub fn compile(source: &str) -> Result<Cpu, Box<dyn Error>> {
        let code = source
            .lines()
            .map(Instruction::from_str)
            .collect::<Result<Vec<Instruction>, _>>()?;
        Ok(Cpu {
            code,
            pc: 0,
            cycle: 1,
            x: 1,
            state: State::BeginNextInstruction,
        })
    }

    /// Run the next cycle, returning the value of X during (not after) the cycle
    pub fn cycle(&mut self) -> i32 {
        let x = self.x;
        match self.state {
            State::BeginNextInstruction => match self.code[self.pc] {
                Instruction::AddX(_) => self.state = State::MidInstruction(1),
                Instruction::Noop => {
                    self.pc += 1;
                    if self.pc == self.code.len() {
                        self.state = State::Complete;
                    } else {
                        self.state = State::BeginNextInstruction
                    }
                }
            },
            State::MidInstruction(remaining_cycles) => {
                if remaining_cycles == 1 {
                    // finish the current instruction
                    self.state = State::BeginNextInstruction;
                    match self.code[self.pc] {
                        Instruction::AddX(v) => self.x += v,
                        Instruction::Noop => {}
                    }

                    // prepare for the next instruction
                    self.pc += 1;
                    if self.pc == self.code.len() {
                        self.state = State::Complete;
                    } else {
                        self.state = State::BeginNextInstruction
                    }
                } else {
                    // count down for this instruction to finish
                    self.state = State::MidInstruction(remaining_cycles - 1);
                }
            }
            State::Complete => {}
        }
        self.cycle += 1;
        x
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Instruction {
    AddX(i32),
    Noop,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(valstr) = s.trim().strip_prefix("addx ") {
            Ok(Instruction::AddX(valstr.parse()?))
        } else if s.trim() == "noop" {
            Ok(Instruction::Noop)
        } else {
            Err(format!("Input instruction '{}' not recognised", s).into())
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    /// Currently executing instruction at program counter, with the indicated number of cycles to go
    MidInstruction(usize),
    /// Next cycle will begin execution of instruction at pc
    BeginNextInstruction,
    /// No more instructions
    Complete,
}

#[test]
fn test_mini_program() {
    let input = "noop
addx 3
addx -5";
    let mut cpu = Cpu::compile(input).expect("This should compile");
    assert_eq!(cpu.cycle(), 1);
    assert_eq!(cpu.cycle(), 1);
    assert_eq!(cpu.cycle(), 1);
    assert_eq!(cpu.cycle(), 4);
    assert_eq!(cpu.cycle(), 4);
    assert_eq!(cpu.get_x(), -1);
}
