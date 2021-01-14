use std::collections::{VecDeque,HashMap};
use itertools::Itertools;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::fmt;
use std::time::Duration;
use ndarray::prelude::*;
use colored::*;

pub fn part1(input: String) {
    println!("Day 11, Part 1:");
    let mut program_input = get_program_input(input);
    println!("Length of program input is {}", program_input.len());

    let memory_size = 2048;

    let (host_tx, host_rx) = mpsc::channel();
    let (comp_tx, comp_rx) = mpsc::channel();

    let mut comp = IntCodeInterpreter::new(&mut program_input, memory_size, host_tx.clone(), comp_rx, host_tx.clone());
    let t = thread::spawn(move || {
        comp.run_program();
    });

    let mut current_point = Point { x: 0, y: 0 };
    let mut current_direction = 0;
    let mut covered_ground: HashMap<Point,i64> = HashMap::new();

    comp_tx.send(0);

    loop {
        let col = match host_rx.recv_timeout(Duration::from_millis(400)) {
            Ok(colour) => colour,
            Err(error) => {
                println!("Error receiving colour: {:?}", error);
                break;
            }
        };

        let dir = match host_rx.recv_timeout(Duration::from_millis(400)) {
            Ok(direction) => direction,
            Err(error) => {
                println!("Error receiving direction: {:?}", error);
                break;
            }
        };

        covered_ground.insert(current_point.clone(), col);

        match dir {
            0 => current_direction += 90,
            1 => current_direction -= 90,
            _ => {}
        };

        current_direction = current_direction % 360;
        if current_direction < 0 {
            current_direction += 360;
        }

        println!("Current direction is {}", current_direction);

        match current_direction {
            0 => current_point.y += 1,
            90 => current_point.x -= 1,
            180 => current_point.y -= 1,
            270 => current_point.x += 1,
            _ => println!("Should not reach this angle!"),
        }

        match covered_ground.get(&current_point) {
            Some(col) => comp_tx.send(*col),
            None => comp_tx.send(0),
        };

    }

    println!("Finished painting");
    println!("Painted {} unique tiles", covered_ground.len());
}

pub fn part2(input: String) {
    println!("Day 11, Part 2:");
    let mut program_input = get_program_input(input);
    println!("Length of program input is {}", program_input.len());

    let memory_size = 2048;

    let (host_tx, host_rx) = mpsc::channel();
    let (comp_tx, comp_rx) = mpsc::channel();

    let mut comp = IntCodeInterpreter::new(&mut program_input, memory_size, host_tx.clone(), comp_rx, host_tx.clone());
    let t = thread::spawn(move || {
        comp.run_program();
    });

    let mut current_point = Point { x: 0, y: 0 };
    let mut current_direction = 0;
    let mut covered_ground: HashMap<Point,i64> = HashMap::new();

    comp_tx.send(1);

    loop {
        let col = match host_rx.recv_timeout(Duration::from_millis(400)) {
            Ok(colour) => colour,
            Err(error) => {
                println!("Error receiving colour: {:?}", error);
                break;
            }
        };

        let dir = match host_rx.recv_timeout(Duration::from_millis(400)) {
            Ok(direction) => direction,
            Err(error) => {
                println!("Error receiving direction: {:?}", error);
                break;
            }
        };

        covered_ground.insert(current_point.clone(), col);

        match dir {
            0 => current_direction += 90,
            1 => current_direction -= 90,
            _ => {}
        };

        current_direction = current_direction % 360;
        if current_direction < 0 {
            current_direction += 360;
        }

        // println!("Current direction is {}", current_direction);

        match current_direction {
            0 => current_point.y += 1,
            90 => current_point.x -= 1,
            180 => current_point.y -= 1,
            270 => current_point.x += 1,
            _ => println!("Should not reach this angle!"),
        }

        match covered_ground.get(&current_point) {
            Some(col) => comp_tx.send(*col),
            None => comp_tx.send(0),
        };

    }

    let mut max_point = Point { x: 0, y: 0 };
    let mut min_point = Point { x: 0, y: 0 };

    for (k, _) in covered_ground.iter() {
        match k.x {
            x if x > max_point.x => max_point.x = x,
            x if x < min_point.x => min_point.x = x,
            _ => {},
        }

        match k.y {
            y if y > max_point.y => max_point.y = y,
            y if y < min_point.y => min_point.y = y,
            _ => {},
        }
    }

    let array_x = max_point.x - min_point.x;
    let array_y = max_point.y - min_point.y;

    let mut img = Array::zeros((array_y as usize + 1, array_x as usize + 1));

    for (k, v) in covered_ground.iter() {
        // println!("{}", k);
        img[[(k.y - min_point.y) as usize, (k.x - min_point.x) as usize]] = *v;
    }

    for i in (0..img.nrows()).rev() {
        for j in 0..img.ncols() {
            if img[[i,j]] == 0 {
                print!("{}", "▄".black());
            }
            else {
                print!("{}", "▄".white());
            }
            // print!("{}", final_image[[i, j]]);
        }
        println!("");
    }

    println!("Max point: {:?}, Min point: {:?}", max_point, min_point);

    println!("Finished painting");
    println!("Painted {} unique tiles", covered_ground.len());
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
    relative_base: i64,
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
            relative_base: 0i64,
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
                    let p1 = self.calculate_argument_address(inst.mode1, 1);
                    let p2 = self.calculate_argument_address(inst.mode2, 2);
                    let p3 = self.calculate_argument_address(inst.mode3, 3);

                    // println!("Add function: [{}] + [{}] -> [{}]", p1, p2, p3);
                    self.memory[p3] = self.memory[p1] + self.memory[p2];

                    self.ip += 4;
                },
                OpCode::Multiply => {
                    let p1 = self.calculate_argument_address(inst.mode1, 1);
                    let p2 = self.calculate_argument_address(inst.mode2, 2);
                    let p3 = self.calculate_argument_address(inst.mode3, 3);

                    // println!("Multiply function: [{}] * [{}] -> [{}]", p1, p2, p3);
                    self.memory[p3] = self.memory[p1] * self.memory[p2];
                    
                    self.ip += 4;
                },
                OpCode::Input => {
                    let p1 = self.calculate_argument_address(inst.mode1, 1);
                    let inp = self.rx.recv().unwrap();
                    // println!("Input function: {} -> [{}]", inp, a1);
                    self.memory[p1] = inp;

                    self.ip += 2
                },
                OpCode::Output => {
                    let p1 = self.calculate_argument_address(inst.mode1, 1);
                    // println!("Output: {}", a1);

                    match self.tx.send(self.memory[p1]) {
                        Ok(_a) => {},
                        Err(_e) => {
                            self.host_tx.send(self.memory[p1]).expect("Host TX failed");
                        }

                    }

                    self.ip += 2;
                },
                OpCode::JumpIfTrue => {
                    let p1 = self.calculate_argument_address(inst.mode1, 1);
                    let p2 = self.calculate_argument_address(inst.mode2, 2);

                    if self.memory[p1] != 0 {
                        self.ip = self.memory[p2] as usize;
                    }
                    else {
                        self.ip += 3;
                    }
                },
                OpCode::JumpIfFalse => {
                    let p1 = self.calculate_argument_address(inst.mode1, 1);
                    let p2 = self.calculate_argument_address(inst.mode2, 2);

                    if self.memory[p1] == 0 {
                        self.ip = self.memory[p2] as usize;
                    }
                    else {
                        self.ip += 3;
                    }
                },
                OpCode::LessThan => {
                    let p1 = self.calculate_argument_address(inst.mode1, 1);
                    let p2 = self.calculate_argument_address(inst.mode2, 2);
                    let p3 = self.calculate_argument_address(inst.mode3, 3);

                    if self.memory[p1] < self.memory[p2] {
                        self.memory[p3] = 1;
                    }
                    else {
                        self.memory[p3] = 0;
                    }
                    self.ip += 4;
                },
                OpCode::Equals => {
                    let p1 = self.calculate_argument_address(inst.mode1, 1);
                    let p2 = self.calculate_argument_address(inst.mode2, 2);
                    let p3 = self.calculate_argument_address(inst.mode3, 3);

                    if self.memory[p1] == self.memory[p2] {
                        self.memory[p3] = 1;
                    }
                    else {
                        self.memory[p3] = 0;
                    }
                    self.ip += 4;
                },
                OpCode::RelativeBaseOffset => {
                    let p1 = self.calculate_argument_address(inst.mode1, 1);

                    self.relative_base += self.memory[p1];
                    self.ip += 2;
                }
                OpCode::Halt => {
                    println!("Halt");
                    self.halted = true;
                    break;
                }
            }
        }
        // for i in 0..10 {
        //     print!("{} ", self.memory[i]);
        // }
        // println!("");
    }

    fn calculate_argument_address(&self, instruction_mode: ParamMode, instruction_number: i64) -> usize {
        match instruction_mode {
            ParamMode::Immediate => self.ip + instruction_number as usize,
            ParamMode::Position => self.memory[self.ip + instruction_number as usize] as usize,
            ParamMode::Relative => (self.relative_base + self.memory[self.ip + instruction_number as usize]) as usize,
        }
    }

    fn calculate_argument(&self, instruction_mode: ParamMode, instruction_number: i64) -> i64 {
        match instruction_mode {
            ParamMode::Immediate => self.memory[self.ip + instruction_number as usize],
            ParamMode::Position => self.memory[self.memory[self.ip + instruction_number as usize] as usize],
            ParamMode::Relative => self.relative_base + self.memory[self.ip + instruction_number as usize],
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
    RelativeBaseOffset, // 9
    Halt // 99
}

#[derive(Eq, PartialEq, Debug)]
enum ParamMode {
    Position, // 0
    Immediate, // 1
    Relative, // 2
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
                2 => ParamMode::Relative,
                1 => ParamMode::Immediate,
                _ => ParamMode::Position,
            };

            if dig.len() > 3 {
                m2 = match dig[3] {
                    2 => ParamMode::Relative,
                    1 => ParamMode::Immediate,
                    _ => ParamMode::Position,
                };

                if dig.len() > 4 {
                    m3 = match dig[4] {
                        2 => ParamMode::Relative,
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
            9 => OpCode::RelativeBaseOffset,
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


#[derive(PartialEq, Hash, Eq, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}
