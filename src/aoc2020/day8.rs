use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::character::complete::{char, digit1};
use nom::lib::std::collections::HashSet;
use nom::IResult;

use aoc2020::day8::INSTRUCTION::{ACC, JMP, NOP};
use std::ops::Neg;

/// --- Day 8: Handheld Halting ---
///
/// Your flight to the major airline hub reaches cruising altitude without incident. While you
/// consider checking the in-flight menu for one of those drinks that come with a little umbrella,
/// you are interrupted by the kid sitting next to you.
///
/// Their handheld game console won't turn on! They ask if you can take a look.
///
/// You narrow the problem down to a strange infinite loop in the boot code (your puzzle input) of
/// the device. You should be able to fix it, but first you need to be able to run the code in
/// isolation.
///
/// The boot code is represented as a text file with one instruction per line of text. Each
/// instruction consists of an operation (`acc`, `jmp`, or `nop`) and an argument (a signed number
/// like `+4` or `-20`).
///
///  - `acc` increases or decreases a single global value called the accumulator by the value given
///     in the argument. For example, acc +7 would increase the accumulator by 7. The accumulator
///     starts at 0. After an acc instruction, the instruction immediately below it is executed next.
///  - `jmp` jumps to a new instruction relative to itself. The next instruction to execute is found
///     using the argument as an offset from the `jmp` instruction; for example, `jmp +2` would skip
///     the next instruction, `jmp +1` would continue to the instruction immediately below it, and
///     `jmp -20` would cause the instruction 20 lines above to be executed next.
///  - `nop` stands for No OPeration - it does nothing. The instruction immediately below it is
///     executed next.
///
/// For example, consider the following program:
///
/// ```
/// nop +0
/// acc +1
/// jmp +4
/// acc +3
/// jmp -3
/// acc -99
/// acc +1
/// jmp -4
/// acc +6
/// ```
///
/// These instructions are visited in this order:
///
/// ```
/// nop +0  | 1
/// acc +1  | 2, 8(!)
/// jmp +4  | 3
/// acc +3  | 6
/// jmp -3  | 7
/// acc -99 |
/// acc +1  | 4
/// jmp -4  | 5
/// acc +6  |
/// ```
///
/// First, the `nop +0` does nothing. Then, the accumulator is increased from 0 to 1 (`acc +1`) and
/// `jmp +4` sets the next instruction to the other `acc +1` near the bottom. After it increases the
/// accumulator from 1 to 2, `jmp -4` executes, setting the next instruction to the only `acc +3`.
/// It sets the accumulator to 5, and `jmp -3` causes the program to continue back at the first
/// `acc +1`.
///
/// This is an infinite loop: with this sequence of jumps, the program will run forever. The moment
/// the program tries to run any instruction a second time, you know it will never terminate.
///
/// Immediately before the program would run an instruction a second time, the value in the
/// accumulator is 5.
///
/// Run your copy of the boot code. Immediately before any instruction is executed a second time,
/// what value is in the accumulator?
///
/// --- Part Two ---
///
/// After some careful analysis, you believe that exactly one instruction is corrupted.
///
/// Somewhere in the program, either a `jmp` is supposed to be a `nop`, or a `nop` is supposed to
/// be a `jmp`. (No `acc` instructions were harmed in the corruption of this boot code.)
///
/// The program is supposed to terminate by attempting to execute an instruction immediately after
/// the last instruction in the file. By changing exactly one `jmp` or `nop`, you can repair the
/// boot code and make it terminate correctly.
///
/// For example, consider the same program from above:
///
/// ```
/// nop +0
/// acc +1
/// jmp +4
/// acc +3
/// jmp -3
/// acc -99
/// acc +1
/// jmp -4
/// acc +6
/// ```
///
/// If you change the first instruction from `nop +0` to `jmp +0`, it would create a
/// single-instruction infinite loop, never leaving that instruction.
/// If you change almost any of the `jmp` instructions, the program will still eventually find
/// another `jmp` instruction and loop forever.
///
/// However, if you change the second-to-last instruction (from `jmp -4` to `nop -4`), the program
/// terminates! The instructions are visited in this order:
///
/// ```
/// nop +0  | 1
/// acc +1  | 2
/// jmp +4  | 3
/// acc +3  |
/// jmp -3  |
/// acc -99 |
/// acc +1  | 4
/// nop -4  | 5
/// acc +6  | 6
/// ```
///
/// After the last instruction (`acc +6`), the program terminates by attempting to run the
/// instruction below the last instruction in the file. With this change, after the program
/// terminates, the accumulator contains the value 8 (`acc +1`, `acc +1`, `acc +6`).
///
/// Fix the program so that it terminates normally by changing exactly one `jmp` (to `nop`) or `nop`
/// (to `jmp`). What is the value of the accumulator after the program terminates?

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Copy, Clone)]
enum INSTRUCTION {
    NOP(isize),
    ACC(isize),
    JMP(isize),
}

pub fn parse_int(input: &str) -> IResult<&str, isize> {
    let (input, sign) = alt((char('-'), char('+')))(input)?;
    let (input, digits) = digit1(input)?;

    let num: isize = digits.parse().unwrap();

    Ok((
        input,
        match sign {
            '+' => num,
            '-' => num.neg(),
            _ => {
                panic!("wut")
            }
        },
    ))
}

fn parse_instruction(input: &str) -> IResult<&str, INSTRUCTION> {
    let (input, instruction) = take_until(" ")(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, arg) = parse_int(input)?;

    let instr = match instruction {
        "nop" => NOP(arg),
        "acc" => ACC(arg),
        "jmp" => JMP(arg),
        _ => panic!("unknown instruction: {}", instruction),
    };

    Ok((input, instr))
}

#[derive(Debug, PartialEq)]
struct GameConsole {
    pc: usize,
    acc: isize,
    rom: Vec<INSTRUCTION>,
}

impl GameConsole {
    pub fn new(rom: Vec<INSTRUCTION>) -> GameConsole {
        GameConsole { pc: 0, acc: 0, rom }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
        // println!("RESET");
    }

    pub fn apply_patch(
        &mut self,
        patch_index: usize,
        patch_instruction: INSTRUCTION,
    ) -> INSTRUCTION {
        let original_instr = self.rom[patch_index];
        // println!(
        //     "PATCHING: [{}] {:?} -> {:?}",
        //     patch_index, original_instr, patch_instruction
        // );
        self.rom[patch_index] = patch_instruction;

        original_instr
    }

    pub fn run(&mut self) -> (bool, Vec<(usize, INSTRUCTION)>) {
        let mut potential_patches = Vec::new();

        let mut seen_program_counters: HashSet<usize> = HashSet::new();
        while !seen_program_counters.contains(&self.pc) && self.pc != self.rom.len() {
            seen_program_counters.insert(self.pc);
            let instr = &self.rom[self.pc];
            // println!(
            //     "{:?} :: {:?}",
            //     self, instr
            // );
            match instr {
                NOP(arg) => {
                    potential_patches.push((self.pc, JMP(*arg)));
                    self.pc += 1;
                }
                ACC(arg) => {
                    self.acc += arg;
                    self.pc += 1;
                }
                JMP(arg) => {
                    potential_patches.push((self.pc, NOP(*arg)));
                    if arg.is_positive() {
                        self.pc += arg.abs() as usize;
                    } else {
                        self.pc -= arg.abs() as usize;
                    }
                }
            }
        }

        let finished_successfully = self.pc == self.rom.len();
        // println!(
        //     "Finished, success = {}, potential_patches = {:?}",
        //     finished_successfully, potential_patches
        // );

        (finished_successfully, potential_patches)
    }
}

pub fn solve_part_one(input: &[String]) -> isize {
    let instructions: Vec<INSTRUCTION> = input
        .iter()
        .map(|i| parse_instruction(i).map(|(_, instr)| instr).unwrap())
        .collect();

    let mut console = GameConsole::new(instructions);
    console.run();

    console.acc
}

pub fn solve_part_two(input: &[String]) -> isize {
    let instructions: Vec<INSTRUCTION> = input
        .iter()
        .map(|i| parse_instruction(i).map(|(_, instr)| instr).unwrap())
        .collect();

    let mut console = GameConsole::new(instructions);
    let (finished_successfully, potential_patches) = console.run();
    assert!(
        !finished_successfully,
        "The first run should not finish successfully"
    );

    // Try each patch one at a time
    for (patch_index, patched_instruction) in potential_patches {
        let original_instruction = console.apply_patch(patch_index, patched_instruction);

        console.reset();
        let (finished_successfully, _) = console.run();
        if finished_successfully {
            return console.acc;
        }

        console.apply_patch(patch_index, original_instruction);
    }

    panic!("Could not find a patch that got the boot rom to succeed");
}

#[test]
fn test_parse_isize() {
    assert_eq!(IResult::Ok(("", 123)), parse_int("+123"));
    assert_eq!(IResult::Ok(("", -123)), parse_int("-123"));
}

#[test]
fn test_parse_instr() {
    assert_eq!(
        IResult::Ok(("", INSTRUCTION::NOP(0))),
        parse_instruction("nop +0")
    );
    assert_eq!(
        IResult::Ok(("", INSTRUCTION::ACC(-123))),
        parse_instruction("acc -123")
    );
    assert_eq!(
        IResult::Ok(("", INSTRUCTION::JMP(456))),
        parse_instruction("jmp +456")
    );
}

#[test]
fn examples_part_one() {
    assert_eq!(
        5,
        solve_part_one(&[
            "nop +0".to_string(),
            "acc +1".to_string(),
            "jmp +4".to_string(),
            "acc +3".to_string(),
            "jmp -3".to_string(),
            "acc -99".to_string(),
            "acc +1".to_string(),
            "jmp -4".to_string(),
            "acc +6".to_string(),
        ])
    );
}

#[test]
fn examples_part_two() {
    assert_eq!(
        8,
        solve_part_two(&[
            "nop +0".to_string(),
            "acc +1".to_string(),
            "jmp +4".to_string(),
            "acc +3".to_string(),
            "jmp -3".to_string(),
            "acc -99".to_string(),
            "acc +1".to_string(),
            "jmp -4".to_string(),
            "acc +6".to_string(),
        ])
    );
}
