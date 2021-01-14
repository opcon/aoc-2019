use itertools::Itertools;
use ndarray::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::io;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;
// use colored::*;

use std::io::{stdout, Write};

use crossterm::{
    execute, queue,
    style::{
        Color, Colorize, Print, PrintStyledContent, ResetColor, SetBackgroundColor,
        SetForegroundColor,
    },
    terminal, ExecutableCommand, QueueableCommand, Result,
};

pub fn part1(input: String) {
    println!("Day 13, Part 1:");
    let mut program_input = get_program_input(input);
    println!("Length of program input is {}", program_input.len());

    let memory_size = 2048 * 4;

    let (host_tx, host_rx) = mpsc::channel();
    let (comp_tx, comp_rx) = mpsc::channel();

    let mut comp = IntCodeInterpreter::new(
        &mut program_input,
        memory_size,
        host_tx.clone(),
        comp_rx,
        host_tx.clone(),
    );
    let t = thread::spawn(move || {
        comp.run_program();
    });

    let mut current_point = Point { x: 0, y: 0 };
    let mut current_direction = 0;

    // comp_tx.send(0);
    let mut block_tiles = 0;
    let mut score = 0;

    let mut playing_grid: HashMap<Point, TileID> = HashMap::new();

    loop {
        let x = match host_rx.recv_timeout(Duration::from_millis(400)) {
            Ok(x) => x,
            Err(error) => {
                println!("Error receiving x-coord: {:?}", error);
                break;
            }
        };

        let y = match host_rx.recv_timeout(Duration::from_millis(400)) {
            Ok(y) => y,
            Err(error) => {
                println!("Error receiving y-coord: {:?}", error);
                break;
            }
        };

        if (x == -1) && (y == 0) {
            score = match host_rx.recv_timeout(Duration::from_millis(400)) {
                Ok(score) => score,
                Err(error) => {
                    println!("Error receiving score: {:?}", error);
                    break;
                }
            };
            continue;
        }

        let tile_id = match host_rx.recv_timeout(Duration::from_millis(400)) {
            Ok(tid) => TileID::get_tile_id(tid),
            Err(error) => {
                println!("Error receiving tile id: {:?}", error);
                break;
            }
        };

        playing_grid.insert(Point { x, y }, tile_id);

        match tile_id {
            TileID::Block => block_tiles += 1,
            _ => {}
        }

        // println!("Received tile: (x,y,tid) = ({}, {}, {:?})", x, y, tile_id);
    }

    let mut max_point = Point { x: 0, y: 0 };
    let mut min_point = Point { x: 0, y: 0 };

    for (k, _) in playing_grid.iter() {
        match k.x {
            x if x > max_point.x => max_point.x = x,
            x if x < min_point.x => min_point.x = x,
            _ => {}
        }

        match k.y {
            y if y > max_point.y => max_point.y = y,
            y if y < min_point.y => min_point.y = y,
            _ => {}
        }
    }

    let array_x = max_point.x - min_point.x;
    let array_y = max_point.y - min_point.y;

    println!("Min point: {}, Max point: {}", min_point, max_point);
    println!("array_x: {}, array_y: {}", array_x, array_y);

    let mut img = Array::from_elem((array_y as usize + 1, array_x as usize + 1), TileID::Empty);

    for (k, v) in playing_grid.iter() {
        // println!("{}", k);
        img[[(k.y - min_point.y) as usize, (k.x - min_point.x) as usize]] = *v;
    }

    for i in (0..img.nrows()) {
        for j in 0..img.ncols() {
            match img[[i, j]] {
                TileID::Block => print!("{}", "▄".white()),
                TileID::HorPaddle => print!("{}", "="),
                TileID::Ball => print!("{}", "*"),
                TileID::Wall => print!("{}", "|"),
                TileID::Empty => print!("{}", "▄".black()),
            }
        }
        println!("");
    }

    println!("Finished drawing");
    println!("Drew {} block tiles", block_tiles);
}

pub fn part2(input: String) -> Result<()> {
    println!("Day 13, Part 2:");
    let mut program_input = get_program_input(input);
    println!("Length of program input is {}", program_input.len());

    let memory_size = 2048 * 4;

    let (host_tx, host_rx) = mpsc::channel();
    let (comp_tx, comp_rx) = mpsc::channel();

    program_input[0] = 2;

    let mut comp = IntCodeInterpreter::new(
        &mut program_input,
        memory_size,
        host_tx.clone(),
        comp_rx,
        host_tx.clone(),
    );
    let t = thread::spawn(move || {
        comp.run_program();
    });

    let mut stdout = stdout();

    // comp_tx.send(0);
    let mut score = 0;

    let max_point = Point { x: 39, y: 23 };
    let min_point = Point { x: 0, y: 0 };

    let array_x = max_point.x - min_point.x;
    let array_y = max_point.y - min_point.y;

    let mut playing_grid =
        Array::from_elem((array_y as usize + 1, array_x as usize + 1), TileID::Empty);

    let mut player_pos = Point { x: 20, y: 22 };
    let mut ball_pos = Point { x: 0, y: 0 };
    let mut ball_vel = Point { x: 0, y: 0 };

    let mut start_game = false;

    loop {
        let x = match host_rx.recv_timeout(Duration::from_millis(400)) {
            Ok(x) => x,
            Err(error) => {
                println!("Error receiving x-coord: {:?}", error);
                break;
            }
        };

        let y = match host_rx.recv_timeout(Duration::from_millis(400)) {
            Ok(y) => y,
            Err(error) => {
                println!("Error receiving y-coord: {:?}", error);
                break;
            }
        };

        if (x == -1) && (y == 0) {
            score = match host_rx.recv_timeout(Duration::from_millis(400)) {
                Ok(score) => score,
                Err(error) => {
                    println!("Error receiving score: {:?}", error);
                    break;
                }
            };
            continue;
        }

        let tile_id = match host_rx.recv_timeout(Duration::from_millis(400)) {
            Ok(tid) => TileID::get_tile_id(tid),
            Err(error) => {
                println!("Error receiving tile id: {:?}", error);
                break;
            }
        };

        playing_grid[[y as usize, x as usize]] = tile_id;

        if (tile_id == TileID::HorPaddle) && !start_game {
            start_game = true;
            comp_tx.send(0); // Neutral
        }

        // Update ball or player positions
        match tile_id {
            TileID::HorPaddle => {
                println!("Updated player position: ({}, {})", x, y);
                player_pos.x = x;
                player_pos.y = y;
            }
            TileID::Ball => {
                println!("Updated ball position: ({}, {})", x, y);
                ball_vel.x = x - ball_pos.x;
                ball_vel.y = y - ball_pos.y;
                ball_pos.x = x;
                ball_pos.y = y;
            }
            _ => {}
        }

        if start_game {
            // send input
            if tile_id == TileID::Ball {
                println!("Changing paddle position");
                if (player_pos.x < ball_pos.x) {
                    comp_tx.send(1); // Go right
                } else if player_pos.x > ball_pos.x {
                    comp_tx.send(-1); // Go left
                } else if ball_vel.x < 0 {
                    comp_tx.send(-1); // Go left
                } else if ball_vel.x > 0 {
                    comp_tx.send(1); // Go right
                } else {
                    comp_tx.send(0); // Neutral
                }
            }
        }

        // draw playing board
        print!("\x1B[2J");

        let lock = stdout.lock();

        let mut w = io::BufWriter::new(lock);

        for i in 0..playing_grid.nrows() {
            for j in 0..playing_grid.ncols() {
                match playing_grid[[i, j]] {
                    TileID::Block => write!(&mut w, "▄")?,
                    TileID::HorPaddle => write!(&mut w, "=")?,
                    TileID::Ball => write!(&mut w, "*")?,
                    TileID::Wall => write!(&mut w, "|")?,
                    TileID::Empty => write!(&mut w, " ")?,
                }
            }
            writeln!(&mut w, "")?;
        }
    }

    println!("Score is {}", score);

    Ok(())
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
    fn new(
        program_input: &mut Vec<i64>,
        memory_size: i64,
        tx: Sender<i64>,
        rx: Receiver<i64>,
        host_tx: Sender<i64>,
    ) -> IntCodeInterpreter {
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
                }
                OpCode::Multiply => {
                    let p1 = self.calculate_argument_address(inst.mode1, 1);
                    let p2 = self.calculate_argument_address(inst.mode2, 2);
                    let p3 = self.calculate_argument_address(inst.mode3, 3);

                    // println!("Multiply function: [{}] * [{}] -> [{}]", p1, p2, p3);
                    self.memory[p3] = self.memory[p1] * self.memory[p2];
                    self.ip += 4;
                }
                OpCode::Input => {
                    let p1 = self.calculate_argument_address(inst.mode1, 1);
                    let inp = self.rx.recv().unwrap();
                    // println!("Input function: {} -> [{}]", inp, a1);
                    self.memory[p1] = inp;

                    self.ip += 2
                }
                OpCode::Output => {
                    let p1 = self.calculate_argument_address(inst.mode1, 1);
                    // println!("Output: {}", a1);

                    match self.tx.send(self.memory[p1]) {
                        Ok(_a) => {}
                        Err(_e) => {
                            self.host_tx.send(self.memory[p1]).expect("Host TX failed");
                        }
                    }

                    self.ip += 2;
                }
                OpCode::JumpIfTrue => {
                    let p1 = self.calculate_argument_address(inst.mode1, 1);
                    let p2 = self.calculate_argument_address(inst.mode2, 2);

                    if self.memory[p1] != 0 {
                        self.ip = self.memory[p2] as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                OpCode::JumpIfFalse => {
                    let p1 = self.calculate_argument_address(inst.mode1, 1);
                    let p2 = self.calculate_argument_address(inst.mode2, 2);

                    if self.memory[p1] == 0 {
                        self.ip = self.memory[p2] as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                OpCode::LessThan => {
                    let p1 = self.calculate_argument_address(inst.mode1, 1);
                    let p2 = self.calculate_argument_address(inst.mode2, 2);
                    let p3 = self.calculate_argument_address(inst.mode3, 3);

                    if self.memory[p1] < self.memory[p2] {
                        self.memory[p3] = 1;
                    } else {
                        self.memory[p3] = 0;
                    }
                    self.ip += 4;
                }
                OpCode::Equals => {
                    let p1 = self.calculate_argument_address(inst.mode1, 1);
                    let p2 = self.calculate_argument_address(inst.mode2, 2);
                    let p3 = self.calculate_argument_address(inst.mode3, 3);

                    if self.memory[p1] == self.memory[p2] {
                        self.memory[p3] = 1;
                    } else {
                        self.memory[p3] = 0;
                    }
                    self.ip += 4;
                }
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

    fn calculate_argument_address(
        &self,
        instruction_mode: ParamMode,
        instruction_number: i64,
    ) -> usize {
        match instruction_mode {
            ParamMode::Immediate => self.ip + instruction_number as usize,
            ParamMode::Position => self.memory[self.ip + instruction_number as usize] as usize,
            ParamMode::Relative => {
                (self.relative_base + self.memory[self.ip + instruction_number as usize]) as usize
            }
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
enum OpCode {
    Add,                // 1
    Multiply,           // 2
    Input,              // 3
    Output,             // 4
    JumpIfTrue,         // 5
    JumpIfFalse,        // 6
    LessThan,           // 7
    Equals,             // 8
    RelativeBaseOffset, // 9
    Halt,               // 99
}

#[derive(Eq, PartialEq, Debug)]
enum ParamMode {
    Position,  // 0
    Immediate, // 1
    Relative,  // 2
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
        let (mut m1, mut m2, mut m3) = (
            ParamMode::Position,
            ParamMode::Position,
            ParamMode::Position,
        );

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
            _ => dig[0] + dig[1] * 10,
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
            _ => {
                println!("INVALID OPCODE");
                OpCode::Halt
            }
        };

        Instruction {
            opcode: op_enum,
            mode1: m1,
            mode2: m2,
            mode3: m3,
        }
    }
}

fn get_program_input(input: String) -> Vec<i64> {
    input
        .split(',')
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
    x: i64,
    y: i64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum TileID {
    Empty,     // 0
    Wall,      // 1
    Block,     // 2
    HorPaddle, // 3
    Ball,      // 4
}

impl TileID {
    fn get_tile_id(id: i64) -> TileID {
        match id {
            0 => TileID::Empty,
            1 => TileID::Wall,
            2 => TileID::Block,
            3 => TileID::HorPaddle,
            4 => TileID::Ball,
            a => panic!("Invalid tile id {}", a),
        }
    }
}
