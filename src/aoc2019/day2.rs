pub fn part_1_mangling(input_codes: &mut Vec<isize>) {
    input_codes[1] = 12;
    input_codes[2] = 2;
}

pub fn solve_part_one<F>(input: &[String], mangle: F) -> isize where F: Fn(&mut Vec<isize>) {
    let csv_intcode = input.get(0).unwrap();
    let mut int_codes: Vec<isize> = csv_intcode.split(',').map(|i| i.parse().unwrap()).collect();

    run_vm(&mut int_codes, mangle)
}

fn run_vm<F>(int_codes: &mut Vec<isize>, mangle: F) -> isize where F: Fn(&mut Vec<isize>) {
    mangle(int_codes);

    let mut pc = 0;

    loop {
//        println!("State: PC={} MEM={:?}", pc, int_codes);
        let opcode = int_codes[pc];

        match opcode {
            1 => {
                let arg_pos_1 = int_codes[pc + 1] as usize;
                let arg_pos_2 = int_codes[pc + 2] as usize;
                let res_pos = int_codes[pc + 3] as usize;

                let arg1 = int_codes[arg_pos_1];
                let arg2 = int_codes[arg_pos_2];

                int_codes[res_pos] = arg1 + arg2;

                pc += 4;
            }
            2 => {
                let arg_pos_1 = int_codes[pc + 1] as usize;
                let arg_pos_2 = int_codes[pc + 2] as usize;
                let res_pos = int_codes[pc + 3] as usize;

                let arg1 = int_codes[arg_pos_1];
                let arg2 = int_codes[arg_pos_2];

                int_codes[res_pos] = arg1 * arg2;

                pc += 4;
            }
            99 => break,
            _ => panic!("Found unexpected opcode: {}", opcode),
        }
    }

//    println!("State: PC={} MEM={:?}", pc, int_codes);

    int_codes[0]
}

pub fn solve_part_two(input: &[String]) -> isize {
    let csv_intcode = input.get(0).unwrap();
    let int_codes: Vec<isize> = csv_intcode.split(',').map(|i| i.parse().unwrap()).collect();

    let mut answers: Vec<(isize, isize)> = Vec::new();

    for i in 0..=99 {
        for j in 0..=99 {
            let mut cloned_codes = int_codes.clone();
            let result = run_vm(&mut cloned_codes, |ic| {
                ic[1] = i;
                ic[2] = j
            });

            if result == 19690720 {
                return 100 * i + j;
//                answers.push((i, j));
            }
        }
    }

//    println!("Answers: {:?}", answers);

    -1
}

#[test]
fn examples_part_one() {
    assert_eq!(
        3500,
        solve_part_one(&["1,9,10,3,2,3,11,0,99,30,40,50".to_string()], |x|{})
    );
    assert_eq!(2, solve_part_one(&["1,0,0,0,99".to_string()], |x|{}));
    assert_eq!(2, solve_part_one(&["2,3,0,3,99".to_string()], |x|{}));
    assert_eq!(2, solve_part_one(&["2,4,4,5,99,0".to_string()], |x|{}));
    assert_eq!(
        30,
        solve_part_one(&["1,1,1,4,99,5,6,0,99".to_string()], |x|{})
    );
}

#[test]
fn examples_part_two() {}
