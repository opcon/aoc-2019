use std::collections::VecDeque;
use itertools::Itertools;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

pub fn part1(input: String) {
    println!("Day 7, Part 1:");
    let mut program_input = get_program_input(input);
    println!("Length of program input is {}", program_input.len());

    let mut max_output = 0i64;
    let mut max_phase = Vec::new();

    let phase_options = vec![0, 1, 2, 3, 4].into_iter();

    // for (i, j, k, l, m) in iproduct!(0..5, 0..5, 0..5, 0..5, 0..5) {
    for phase in phase_options.permutations(5) {

        let mut inp: VecDeque<i64> = VecDeque::new();
        let mut output = vec![0i64];

        for i in 0..5 {
            inp.push_back(phase[i]);
            inp.push_back(output[0]);

            output = run_intcode(&mut program_input, &mut inp);
            // println!("Output from amplifier {} is {}", i, output[0]);
        }

        if output[0] > max_output {
            max_output = output[0];
            max_phase = phase.to_vec();
        }
    }

    println!("Maximum output is {} for phases {:?}", max_output, max_phase);

}

pub fn part2(input: String) {
    println!("Day 7, Part 2:");
    let mut program_input = get_program_input(input);
    println!("Length of program input is {}", program_input.len());

    let mut max_output = 0i64;
    let mut max_phase = Vec::new();

    let phase_options = vec![5i64, 6i64, 7i64, 8i64, 9i64];

    // for (i, j, k, l, m) in iproduct!(0..5, 0..5, 0..5, 0..5, 0..5) {
    for phase in phase_options.into_iter().permutations(5) {

        println!("{:?}", phase);
        let mut inp: VecDeque<i64> = VecDeque::new();

        let output = run_with_new_phase(&mut program_input, &phase);

        if output > max_output {
            max_output = output;
            max_phase = phase.to_vec();
        }
    }

    println!("Maximum output is {} for phases {:?}", max_output, max_phase);

    // run_with_new_phase(&mut program_input, &vec![9, 7, 8, 5, 6]);

    // let (tx0, rx0): (Sender<i64>, Receiver<i64>) = mpsc::channel();
    // let mut comp0 = IntCodeInterpreter::new(&mut program_input, 2048, tx0, rx0);

}

fn run_with_new_phase(program_input: &mut Vec<i64>, phase: & Vec<i64>) -> i64 {

    let memory_size = 2048;

    let (mut master_tx, mut master_rx) = mpsc::channel();
    let mut tx_channels = Vec::new();
    let mut rx_channels = Vec::new();

    // let mut channels: Vec<(Sender<i64>, Receiver<i64>)> = Vec::new();
    let mut computers: Vec<IntCodeInterpreter> = Vec::new();

    for _i in 0..5 {
        let (tx, rx) = mpsc::channel();
        tx_channels.push(tx);
        rx_channels.push(rx);
    }

    let start_tx = tx_channels[4].clone();

    for i in 0..5 {
        if i == 0 {
            // tx channel 0
            // rx channel 4
            computers.push(IntCodeInterpreter::new(program_input, memory_size, tx_channels[0].clone(), rx_channels.pop().unwrap(), master_tx.clone()));
            rx_channels.reverse();
        } else if i == 4 {
            // tx channel 4
            // rx channel 3
            computers.push(IntCodeInterpreter::new(program_input, memory_size, tx_channels[4].clone(), rx_channels.pop().unwrap(), master_tx.clone()));
        } else {

            // connect tx channel n
            // connect rx channel n-1
            computers.push(IntCodeInterpreter::new(program_input, memory_size, tx_channels[i].clone(), rx_channels.pop().unwrap(), master_tx.clone()));
        }
    }

    // Run the code
    computers.reverse();

    start_tx.send(phase[0]);
    start_tx.send(0);

    let mut threads = Vec::new();
    for i in 0..5 {
        if i != 0 {
            tx_channels[i-1].send(phase[i]);
        }
        let mut c = computers.pop().unwrap();
        let t = thread::Builder::new().name(i.to_string()).spawn(move || {
            c.run_program();
        }).unwrap();
        threads.push(t);
    }

    for t in threads {
        t.join().expect("thread failed");
    }

    let output = master_rx.recv().unwrap();
    println!("Output is {}", output);

    output
}

struct IntCodeInterpreter {
    memory: Vec<i64>,
    tx: Sender<i64>,
    rx: Receiver<i64>,
    ip: usize,
    halted: bool,
    host_tx: Sender<i64>,
}

impl IntCodeInterpreter {
    fn new(program_input: &mut Vec<i64>, memory_size: i64, tx: Sender<i64>, rx: Receiver<i64>, host_tx: Sender<i64>) -> IntCodeInterpreter {
        let mut memory = program_input.to_vec(); //vec![0; memory_size as usize];
        memory.resize(memory_size as usize, 0i64);

        IntCodeInterpreter {
            memory,
            rx,
            tx,
            ip: 0,
            halted: false,
            host_tx,
        }
    }

    fn run_program(&mut self) {
        // this is our main loop
        loop {
            // Check that there is more program left to execute
            if self.ip >= self.memory.len() {
                println!("Ran out of program input! Something went wrong.");
                break;
            }

            // Match on our opcode
            let inst = Instruction::parse_opcode(self.memory[self.ip]);
            // println!("{:?}", inst);

            match inst.opcode {
                OpCode::Add => {
                    let a1 = match inst.mode1 {
                        ParamMode::Immediate => self.memory[self.ip + 1],
                        ParamMode::Position => self.memory[self.memory[self.ip + 1] as usize],
                    };
                    let a2 = match inst.mode2 {
                        ParamMode::Immediate => self.memory[self.ip + 2],
                        ParamMode::Position => self.memory[self.memory[self.ip + 2] as usize],
                    };
                    let a3 = self.memory[self.ip + 3];

                    // println!("Add function: {} + {} -> [{}]", a1, a2, a3);
                    self.memory[a3 as usize] = a1 + a2;

                    self.ip += 4;
                },
                OpCode::Multiply => {
                    let a1 = match inst.mode1 {
                        ParamMode::Immediate => self.memory[self.ip + 1],
                        ParamMode::Position => self.memory[self.memory[self.ip + 1] as usize],
                    };
                    let a2 = match inst.mode2 {
                        ParamMode::Immediate => self.memory[self.ip + 2],
                        ParamMode::Position => self.memory[self.memory[self.ip + 2] as usize],
                    };
                    let a3 = self.memory[self.ip + 3];

                    // println!("Multiply function: {} * {} -> [{}]", a1, a2, a3);
                    self.memory[a3 as usize] = a1 * a2;
                    
                    self.ip += 4;
                },
                OpCode::Input => {
                    let a1 = self.memory[self.ip + 1];
                    let inp = self.rx.recv().unwrap();
                    // println!("Input function: {} -> [{}]", inp, a1);
                    self.memory[a1 as usize] = inp;

                    self.ip += 2
                },
                OpCode::Output => {
                    let a1 = match inst.mode1 {
                        ParamMode::Immediate => self.memory[self.ip + 1],
                        ParamMode::Position => self.memory[self.memory[self.ip + 1] as usize],
                    };
                    // println!("Output: {}", a1);

                    match self.tx.send(a1) {
                        Ok(a) => {},
                        Err(e) => {
                            self.host_tx.send(a1).expect("Host TX failed");
                        }

                    }

                    self.ip += 2;
                },
                OpCode::JumpIfTrue => {
                    let a1 = match inst.mode1 {
                        ParamMode::Immediate => self.memory[self.ip + 1],
                        ParamMode::Position => self.memory[self.memory[self.ip + 1] as usize],
                    };
                    let a2 = match inst.mode2 {
                        ParamMode::Immediate => self.memory[self.ip + 2],
                        ParamMode::Position => self.memory[self.memory[self.ip + 2] as usize],
                    };

                    if a1 != 0 {
                        self.ip = a2 as usize;
                    }
                    else {
                        self.ip += 3;
                    }
                },
                OpCode::JumpIfFalse => {
                    let a1 = match inst.mode1 {
                        ParamMode::Immediate => self.memory[self.ip + 1],
                        ParamMode::Position => self.memory[self.memory[self.ip + 1] as usize],
                    };
                    let a2 = match inst.mode2 {
                        ParamMode::Immediate => self.memory[self.ip + 2],
                        ParamMode::Position => self.memory[self.memory[self.ip + 2] as usize],
                    };

                    if a1 == 0 {
                        self.ip = a2 as usize;
                    }
                    else {
                        self.ip += 3;
                    }
                },
                OpCode::LessThan => {
                    let a1 = match inst.mode1 {
                        ParamMode::Immediate => self.memory[self.ip + 1],
                        ParamMode::Position => self.memory[self.memory[self.ip + 1] as usize],
                    };
                    let a2 = match inst.mode2 {
                        ParamMode::Immediate => self.memory[self.ip + 2],
                        ParamMode::Position => self.memory[self.memory[self.ip + 2] as usize],
                    };
                    let a3 = self.memory[self.ip + 3];

                    if a1 < a2 {
                        self.memory[a3 as usize] = 1;
                    }
                    else {
                        self.memory[a3 as usize] = 0;
                    }
                    self.ip += 4;
                },
                OpCode::Equals => {
                    let a1 = match inst.mode1 {
                        ParamMode::Immediate => self.memory[self.ip + 1],
                        ParamMode::Position => self.memory[self.memory[self.ip + 1] as usize],
                    };
                    let a2 = match inst.mode2 {
                        ParamMode::Immediate => self.memory[self.ip + 2],
                        ParamMode::Position => self.memory[self.memory[self.ip + 2] as usize],
                    };
                    let a3 = self.memory[self.ip + 3];

                    if a1 == a2 {
                        self.memory[a3 as usize] = 1;
                    }
                    else {
                        self.memory[a3 as usize] = 0;
                    }
                    self.ip += 4;
                },
                OpCode::Halt => {
                    // println!("Halt");
                    self.halted = true;
                    break;
                }
            }
        }
    }
}

fn run_intcode(code: &mut Vec<i64>, input: &mut VecDeque<i64>) -> Vec<i64> {
    let mut output = Vec::new();
    let mut ip = 0;
    loop {
        // Check that there is more program left to execute
        if ip >= code.len() {
            println!("Ran out of program input! Something went wrong.");
            break;
        }

        // Match on our opcode
        let inst = Instruction::parse_opcode(code[ip]);
        // println!("{:?}", inst);

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

                // println!("Add function: {} + {} -> [{}]", a1, a2, a3);
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

                // println!("Multiply function: {} * {} -> [{}]", a1, a2, a3);
                code[a3 as usize] = a1 * a2;
                
                ip += 4;
            },
            OpCode::Input => {
                let a1 = code[ip + 1];
                let inp = input.pop_front().unwrap();
                // println!("Input function: {} -> [{}]", inp, a1);
                code[a1 as usize] = inp;

                ip += 2
            },
            OpCode::Output => {
                let a1 = match inst.mode1 {
                    ParamMode::Immediate => code[ip + 1],
                    ParamMode::Position => code[code[ip + 1] as usize],
                };
                // println!("Output: {}", a1);

                output.push(a1);

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
                // println!("Halt");
                break;
            }
        }
    }
    output
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
