use util::parse_int_csv;

///

#[derive(Debug, PartialEq)]
enum ParameterMode {
    PositionMode,
    ImmediateMode,
}

impl ParameterMode {
    pub fn get_value(&self, value: isize, memory: &Vec<isize>) -> isize {
        match self {
            ParameterMode::PositionMode => memory[value as usize],
            ParameterMode::ImmediateMode => value,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Halt,
    Add(ParameterMode, ParameterMode, ParameterMode),
    Multiply(ParameterMode, ParameterMode, ParameterMode),
    Input(ParameterMode),
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode, ParameterMode),
}

impl Instruction {
    pub fn get_size(&self) -> usize {
        match self {
            Instruction::Halt => 1,
            Instruction::Add(_, _, _) => 4,
            Instruction::Multiply(_, _, _) => 4,
            Instruction::Input(_) => 2,
            Instruction::Output(_) => 2,
            Instruction::JumpIfTrue(_, _) => 3,
            Instruction::JumpIfFalse(_, _) => 3,
            Instruction::LessThan(_, _, _) => 4,
            Instruction::Equals(_, _, _) => 4,
        }
    }
}

fn parse_parameter_mode(mode: isize) -> ParameterMode {
    match mode {
        0 => ParameterMode::PositionMode,
        1 => ParameterMode::ImmediateMode,
        _ => panic!("Unknown parameter mode: {}", mode),
    }
}

fn parse_opcode(opcode: isize, pc: usize) -> Instruction {
    match opcode % 100 {
        1 => {
            let arg1_mode = (opcode / 100) % 10;
            let arg2_mode = (opcode / 1000) % 10;
            let arg3_mode = (opcode / 10000) % 10;

            Instruction::Add(
                parse_parameter_mode(arg1_mode),
                parse_parameter_mode(arg2_mode),
                parse_parameter_mode(arg3_mode),
            )
        }
        2 => {
            let arg1_mode = (opcode / 100) % 10;
            let arg2_mode = (opcode / 1000) % 10;
            let arg3_mode = (opcode / 10000) % 10;

            Instruction::Multiply(
                parse_parameter_mode(arg1_mode),
                parse_parameter_mode(arg2_mode),
                parse_parameter_mode(arg3_mode),
            )
        }
        3 => {
            let arg1_mode = (opcode / 100) % 10;

            Instruction::Input(parse_parameter_mode(arg1_mode))
        }
        4 => {
            let arg1_mode = (opcode / 100) % 10;

            Instruction::Output(parse_parameter_mode(arg1_mode))
        }
        5 => {
            let arg1_mode = (opcode / 100) % 10;
            let arg2_mode = (opcode / 1000) % 10;

            Instruction::JumpIfTrue(
                parse_parameter_mode(arg1_mode),
                parse_parameter_mode(arg2_mode),
            )
        }
        6 => {
            let arg1_mode = (opcode / 100) % 10;
            let arg2_mode = (opcode / 1000) % 10;

            Instruction::JumpIfFalse(
                parse_parameter_mode(arg1_mode),
                parse_parameter_mode(arg2_mode),
            )
        }
        7 => {
            let arg1_mode = (opcode / 100) % 10;
            let arg2_mode = (opcode / 1000) % 10;
            let arg3_mode = (opcode / 10000) % 10;

            Instruction::LessThan(
                parse_parameter_mode(arg1_mode),
                parse_parameter_mode(arg2_mode),
                parse_parameter_mode(arg3_mode),
            )
        }
        8 => {
            let arg1_mode = (opcode / 100) % 10;
            let arg2_mode = (opcode / 1000) % 10;
            let arg3_mode = (opcode / 10000) % 10;

            Instruction::Equals(
                parse_parameter_mode(arg1_mode),
                parse_parameter_mode(arg2_mode),
                parse_parameter_mode(arg3_mode),
            )
        }
        99 => Instruction::Halt,
        _ => panic!("Unknown opcode: {} @ {}", opcode, pc),
    }
}

fn run_vm<F>(memory: &mut Vec<isize>, input_supplier: F) -> Vec<isize>
where
    F: Fn(&usize) -> isize,
{
    let mut outputs = Vec::new();

    let mut pc = 0;

    loop {
        let opcode = memory[pc];

        let instruction = parse_opcode(opcode, pc);
//        println!(
//            "State: PC={} : INSTR={:?} : MEM={:?}",
//            pc, instruction, memory
//        );

        let mut jumped = false;
        match &instruction {
            Instruction::Halt => break,
            Instruction::Add(a1, a2, a3) => {
                let arg_1 = a1.get_value(memory[pc + 1], memory);
                let arg_2 = a2.get_value(memory[pc + 2], memory);
                let arg_3 = memory[pc + 3]; // Result args are position only

//                println!("ADD: arg_1={}, arg_2={}, arg_3={}", arg_1, arg_2, arg_3);
                memory[arg_3 as usize] = arg_1 + arg_2;
            }
            Instruction::Multiply(a1, a2, a3) => {
                let arg_1 = a1.get_value(memory[pc + 1], memory);
                let arg_2 = a2.get_value(memory[pc + 2], memory);
                let arg_3 = memory[pc + 3]; // Result args are position only

//                println!("MUL: arg_1={}, arg_2={}, arg_3={}", arg_1, arg_2, arg_3);
                memory[arg_3 as usize] = arg_1 * arg_2;
            }
            Instruction::Input(a1) => {
                let arg_1 = memory[pc + 1];
                let input = input_supplier(&pc);

//                println!("INPUT: {:?}", input);
                memory[arg_1 as usize] = input;
            }
            Instruction::Output(a1) => {
                let arg_1 = a1.get_value(memory[pc + 1], memory);

//                println!("OUTPUT: {:?}", arg_1);
                outputs.push(arg_1);
            }
            Instruction::JumpIfTrue(a1, a2) => {
                let arg_1 = a1.get_value(memory[pc + 1], memory);
                let arg_2 = a2.get_value(memory[pc + 2], memory);

//                println!("JMP: arg_1={}, arg_2={}", arg_1, arg_2);
                if arg_1 != 0 {
                    pc = arg_2 as usize;
                    jumped = true;
                }
            }
            Instruction::JumpIfFalse(a1, a2) => {
                let arg_1 = a1.get_value(memory[pc + 1], memory);
                let arg_2 = a2.get_value(memory[pc + 2], memory);

//                println!("JNE: arg_1={}, arg_2={}", arg_1, arg_2);
                if arg_1 == 0 {
                    pc = arg_2 as usize;
                    jumped = true;
                }
            }
            Instruction::LessThan(a1, a2, a3) => {
                let arg_1 = a1.get_value(memory[pc + 1], memory);
                let arg_2 = a2.get_value(memory[pc + 2], memory);
                let arg_3 = memory[pc + 3]; // Result args are position only

//                println!(
//                    "LESSTHAN: arg_1={}, arg_2={}, arg_3={}",
//                    arg_1, arg_2, arg_3
//                );
                memory[arg_3 as usize] = if arg_1 < arg_2 { 1 } else { 0 };
            }
            Instruction::Equals(a1, a2, a3) => {
                let arg_1 = a1.get_value(memory[pc + 1], memory);
                let arg_2 = a2.get_value(memory[pc + 2], memory);
                let arg_3 = memory[pc + 3]; // Result args are position only

//                println!("EQUAL: arg_1={}, arg_2={}, arg_3={}", arg_1, arg_2, arg_3);
                memory[arg_3 as usize] = if arg_1 == arg_2 { 1 } else { 0 };
            }
        }

        if !jumped {
            pc += instruction.get_size()
        }
    }

    outputs
}

pub fn solve_part_one(input: &[String]) -> Vec<isize> {
    let mut memory = parse_int_csv(input.get(0).unwrap());

    run_vm(&mut memory, |_| 1)
}

pub fn solve_part_two<F>(input: &[String], input_supplier: F) -> Vec<isize>
where
    F: Fn(&usize) -> isize,
{
    let mut memory = parse_int_csv(input.get(0).unwrap());

    run_vm(&mut memory, input_supplier)
}

#[test]
fn test_opcode_parse() {
    assert_eq!(
        Instruction::Multiply(
            ParameterMode::PositionMode,
            ParameterMode::ImmediateMode,
            ParameterMode::PositionMode,
        ),
        parse_opcode(1002, 0)
    );
}

#[test]
fn examples_part_one() {
    let outputs: Vec<isize> = Vec::new();
    assert_eq!(outputs, solve_part_one(&["1101,100,-1,4,0".to_string()]));

    let outputs: Vec<isize> = vec![99];
    assert_eq!(outputs, solve_part_one(&["4,2,99".to_string()]));

    let outputs: Vec<isize> = vec![69];
    assert_eq!(outputs, solve_part_one(&["104,69,99".to_string()]));
}

#[test]
fn examples_part_two() {
    // Output `input == 8 ? 1 : 0`
    let input_equals_eight_position_mode = &["3,9,8,9,10,9,4,9,99,-1,8".to_string()];
    assert_eq!(
        vec![0],
        solve_part_two(input_equals_eight_position_mode, |_| 5)
    );
    assert_eq!(
        vec![1],
        solve_part_two(input_equals_eight_position_mode, |_| 8)
    );
    assert_eq!(
        vec![0],
        solve_part_two(input_equals_eight_position_mode, |_| 11)
    );

    // Output `input < 8 ? 1 : 0`
    let input_less_than_eight_position_mode = &["3,9,7,9,10,9,4,9,99,-1,8".to_string()];
    assert_eq!(
        vec![1],
        solve_part_two(input_less_than_eight_position_mode, |_| 5)
    );
    assert_eq!(
        vec![0],
        solve_part_two(input_less_than_eight_position_mode, |_| 8)
    );
    assert_eq!(
        vec![0],
        solve_part_two(input_less_than_eight_position_mode, |_| 11)
    );

    // Output `input == 8 ? 1 : 0`
    let input_equals_eight_immediate = &["3,3,1108,-1,8,3,4,3,99".to_string()];
    assert_eq!(vec![0], solve_part_two(input_equals_eight_immediate, |_| 5));
    assert_eq!(vec![1], solve_part_two(input_equals_eight_immediate, |_| 8));
    assert_eq!(
        vec![0],
        solve_part_two(input_equals_eight_immediate, |_| 11)
    );

    // Output `input < 8 ? 1 : 0`
    let input_less_than_eight_immediate_mode = &["3,3,1107,-1,8,3,4,3,99".to_string()];
    assert_eq!(
        vec![1],
        solve_part_two(input_less_than_eight_immediate_mode, |_| 5)
    );
    assert_eq!(
        vec![0],
        solve_part_two(input_less_than_eight_immediate_mode, |_| 8)
    );
    assert_eq!(
        vec![0],
        solve_part_two(input_less_than_eight_immediate_mode, |_| 11)
    );

    // Output `input != 0 ? 1 : 0`
    let input_not_zero_position_mode = &["3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9".to_string()];
    assert_eq!(
        vec![1],
        solve_part_two(input_not_zero_position_mode, |_| -1)
    );
    assert_eq!(vec![0], solve_part_two(input_not_zero_position_mode, |_| 0));
    assert_eq!(vec![1], solve_part_two(input_not_zero_position_mode, |_| 1));

    // Output `input != 0 ? 1 : 0`
    let input_not_zero_immediate_mode = &["3,3,1105,-1,9,1101,0,0,12,4,12,99,1".to_string()];
    assert_eq!(
        vec![1],
        solve_part_two(input_not_zero_immediate_mode, |_| -1)
    );
    assert_eq!(
        vec![0],
        solve_part_two(input_not_zero_immediate_mode, |_| 0)
    );
    assert_eq!(
        vec![1],
        solve_part_two(input_not_zero_immediate_mode, |_| 1)
    );

    // Output `input != 0 ? 1 : 0`
    let input_compare_to_eight = &["3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99".to_string()];
    assert_eq!(vec![999], solve_part_two(input_compare_to_eight, |_| 1));
    assert_eq!(vec![1000], solve_part_two(input_compare_to_eight, |_| 8));
    assert_eq!(vec![1001], solve_part_two(input_compare_to_eight, |_| 20));
}
