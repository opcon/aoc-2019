pub fn part1(input: String) {
    println!("Day 5, Part 1:");
    let mut program_input = get_program_input(input);
    println!("Length of program input is {}", program_input.len());

    println!("Running intcode program with input 1");
    run_intcode(&mut program_input, 1i64);

}

pub fn part2(input: String) {
    println!("Day 5, Part 2:");
    let mut program_input = get_program_input(input);
    println!("Length of program input is {}", program_input.len());

    println!("Running intcode program with input 5");
    run_intcode(&mut program_input, 5i64);
}

fn run_intcode(code: &mut Vec<i64>, input: i64) {
    let mut ip = 0;
    loop {
        // Check that there is more program left to execute
        if ip >= code.len() {
            println!("Ran out of program input! Something went wrong.");
            break;
        }

        // Match on our opcode
        let inst = Instruction::parse_opcode(code[ip]);
        println!("{:?}", inst);

        match inst.opcode {
            OpCode::Add => {
                let a1 = match inst.mode1 {
                    ParamMode::Immediate => code[ip + 1],
                    ParamMode::Position => code[code[ip + 1] as usize],
                };
                let a2 = match inst.mode2 {
                    ParamMode::Immediate => code[ip + 2],
                    ParamMode::Position => code[code[ip + 2] as usize],
                };
                let a3 = code[ip + 3];

                println!("Add function: {} + {} -> [{}]", a1, a2, a3);
                code[a3 as usize] = a1 + a2;

                ip += 4;
            },
            OpCode::Multiply => {
                let a1 = match inst.mode1 {
                    ParamMode::Immediate => code[ip + 1],
                    ParamMode::Position => code[code[ip + 1] as usize],
                };
                let a2 = match inst.mode2 {
                    ParamMode::Immediate => code[ip + 2],
                    ParamMode::Position => code[code[ip + 2] as usize],
                };
                let a3 = code[ip + 3];

                println!("Multiply function: {} * {} -> [{}]", a1, a2, a3);
                code[a3 as usize] = a1 * a2;
                
                ip += 4;
            },
            OpCode::Input => {
                let a1 = code[ip + 1];
                println!("Input function: {} -> [{}]", input, a1);
                code[a1 as usize] = input;

                ip += 2
            },
            OpCode::Output => {
                let a1 = match inst.mode1 {
                    ParamMode::Immediate => code[ip + 1],
                    ParamMode::Position => code[code[ip + 1] as usize],
                };
                println!("Output: {}", a1);

                ip += 2;
            },
            OpCode::JumpIfTrue => {
                let a1 = match inst.mode1 {
                    ParamMode::Immediate => code[ip + 1],
                    ParamMode::Position => code[code[ip + 1] as usize],
                };
                let a2 = match inst.mode2 {
                    ParamMode::Immediate => code[ip + 2],
                    ParamMode::Position => code[code[ip + 2] as usize],
                };

                if a1 != 0 {
                    ip = a2 as usize;
                }
                else {
                    ip += 3;
                }
            },
            OpCode::JumpIfFalse => {
                let a1 = match inst.mode1 {
                    ParamMode::Immediate => code[ip + 1],
                    ParamMode::Position => code[code[ip + 1] as usize],
                };
                let a2 = match inst.mode2 {
                    ParamMode::Immediate => code[ip + 2],
                    ParamMode::Position => code[code[ip + 2] as usize],
                };

                if a1 == 0 {
                    ip = a2 as usize;
                }
                else {
                    ip += 3;
                }
            },
            OpCode::LessThan => {
                let a1 = match inst.mode1 {
                    ParamMode::Immediate => code[ip + 1],
                    ParamMode::Position => code[code[ip + 1] as usize],
                };
                let a2 = match inst.mode2 {
                    ParamMode::Immediate => code[ip + 2],
                    ParamMode::Position => code[code[ip + 2] as usize],
                };
                let a3 = code[ip + 3];

                if a1 < a2 {
                    code[a3 as usize] = 1;
                }
                else {
                    code[a3 as usize] = 0;
                }
                ip += 4;
            },
            OpCode::Equals => {
                let a1 = match inst.mode1 {
                    ParamMode::Immediate => code[ip + 1],
                    ParamMode::Position => code[code[ip + 1] as usize],
                };
                let a2 = match inst.mode2 {
                    ParamMode::Immediate => code[ip + 2],
                    ParamMode::Position => code[code[ip + 2] as usize],
                };
                let a3 = code[ip + 3];

                if a1 == a2 {
                    code[a3 as usize] = 1;
                }
                else {
                    code[a3 as usize] = 0;
                }
                ip += 4;
            },
            OpCode::Halt => {
                println!("Halt");
                break;
            }
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
enum OpCode {
    Add, // 1
    Multiply, // 2
    Input, // 3
    Output, // 4
    JumpIfTrue, // 5
    JumpIfFalse, // 6
    LessThan, // 7
    Equals, // 8
    Halt // 99
}

#[derive(Eq, PartialEq, Debug)]
enum ParamMode {
    Position,
    Immediate,
}

#[derive(Eq, PartialEq, Debug)]
struct Instruction {
    opcode: OpCode,
    mode1: ParamMode,
    mode2: ParamMode,
    mode3: ParamMode,
}

impl Instruction {
    pub fn parse_opcode(input: i64) -> Instruction {
        let mut dig = number_to_vec(input);
        dig.reverse();
        let (mut m1, mut m2, mut m3) = (ParamMode::Position, ParamMode::Position, ParamMode::Position);

        if dig.len() > 2 {
            m1 = match dig[2] {
                1 => ParamMode::Immediate,
                _ => ParamMode::Position,
            };

            if dig.len() > 3 {
                m2 = match dig[3] {
                    1 => ParamMode::Immediate,
                    _ => ParamMode::Position,
                };

                if dig.len() > 4 {
                    m3 = match dig[4] {
                        1 => ParamMode::Immediate,
                        _ => ParamMode::Position,
                    };
                }
            }
        }

        let op = match dig.len() {
            1 => dig[0],
            _ => dig[0] + dig[1]*10,
        };

        let op_enum = match op {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            99 => OpCode::Halt,
            _ => { println!("INVALID OPCODE"); OpCode::Halt },
        };

        Instruction { 
            opcode: op_enum,
            mode1: m1,
            mode2: m2,
            mode3: m3 
        }
    }
}

fn get_program_input(input: String) -> Vec<i64> {
    input.split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect()
}

fn number_to_vec(n: i64) -> Vec<i64> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);
    digits.reverse();
    digits
}
