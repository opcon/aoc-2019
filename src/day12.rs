use num::integer;
use regex::{Regex, RegexSetBuilder};
use std::cmp::{Ordering, PartialEq};
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Sub};

pub fn part1(input: String) {
    println!("Day 12 Part 1:");

    let mut moons: Vec<Moon> = Vec::new();
    let mut affected_moons: HashMap<Moon, Vec<Moon>> = HashMap::new();

    let rg = Regex::new(r"<x=(?P<x>-?\d+),\sy=(?P<y>-?\d+),\sz=(?P<z>-?\d+)>").unwrap();

    for (i, l) in input.lines().enumerate() {
        match rg.captures(l) {
            Some(r) => {
                moons.push(Moon::new(
                    Vector3 {
                        x: r.name("x").unwrap().as_str().parse().unwrap(),
                        y: r.name("y").unwrap().as_str().parse().unwrap(),
                        z: r.name("z").unwrap().as_str().parse().unwrap(),
                    },
                    i,
                ));
            }
            None => panic!("Shouldn't happen!"),
        }
    }

    let mut count = 0;
    let stop = 1000;

    loop {
        for i in 0..moons.len() {
            for j in 0..moons.len() {
                if i == j {
                    continue;
                }
                // println!("Update gravity on moon {} due to moon {}", i, j);
                let m2 = &moons[j].clone();
                let m1 = &mut moons[i];

                m1.calculate_gravity(&m2);
            }
        }
        for m in moons.iter_mut() {
            (*m).tick();
        }

        count += 1;
        // println!("After {} ticks:", count);
        // for m in moons.iter() {
        //     println!("{}: pos={}, vel={}", m.id, m.position, m.velocity);
        // }
        if count >= stop {
            println!("Stopping after {} ticks", count);
            break;
        }
    }

    let mut total_energy_in_system = 0.;

    println!("Energy:");
    for m in moons.iter_mut() {
        m.calculate_energy();
        println!(
            "{}: pot = {}, kin = {}, tot = {}",
            m.id,
            m.potential_energy,
            m.kinetic_energy,
            m.potential_energy * m.kinetic_energy
        );
        total_energy_in_system += m.potential_energy * m.kinetic_energy;
    }

    println!("Total energy in system is {}", total_energy_in_system);

    // println!("{:?}", moons);
}

pub fn part2(input: String) {
    println!("Day 12 Part 2:");

    let (mut moons_x, mut moons_y, mut moons_z) = (Vec::new(), Vec::new(), Vec::new());
    let rg = Regex::new(r"<x=(?P<x>-?\d+),\sy=(?P<y>-?\d+),\sz=(?P<z>-?\d+)>").unwrap();

    let moon_count = input.lines().count();
    let (mut moons_vx, mut moons_vy, mut moons_vz) = (
        vec![0; moon_count],
        vec![0; moon_count],
        vec![0; moon_count],
    );

    for l in input.lines() {
        match rg.captures(l) {
            Some(r) => {
                moons_x.push(r.name("x").unwrap().as_str().parse::<i32>().unwrap());
                moons_y.push(r.name("y").unwrap().as_str().parse::<i32>().unwrap());
                moons_z.push(r.name("z").unwrap().as_str().parse::<i32>().unwrap());
            }
            None => panic!("Shouldn't happen!"),
        }
    }

    let mut counts = Vec::new();

    for dim in 0..3 {
        let moon_pos = match dim {
            0 => &mut moons_x,
            1 => &mut moons_y,
            2 => &mut moons_z,
            _ => unreachable!(),
        };

        let moon_vel = match dim {
            0 => &mut moons_vx,
            1 => &mut moons_vy,
            2 => &mut moons_vz,
            _ => unreachable!(),
        };

        let moon_pos_initial = moon_pos.to_vec();
        let moon_vel_initial = moon_vel.to_vec();

        let mut counter = 0;
        loop {
            // println!("Step {}, Dimension {}: pos: {:?}, vel: {:?}", counter, dim, moon_pos, moon_vel);

            for (i, m) in moon_pos.iter().enumerate() {
                for other in moon_pos.iter() {
                    match m.cmp(other) {
                        Ordering::Less => moon_vel[i] += 1,
                        Ordering::Equal => {}
                        Ordering::Greater => moon_vel[i] -= 1,
                    };
                }
            }

            for i in 0..moon_count {
                moon_pos[i] += moon_vel[i];
            }

            counter += 1;

            if (moon_pos_initial
                .iter()
                .zip(moon_pos.iter())
                .all(|(a, b)| a == b))
                && (moon_vel_initial
                    .iter()
                    .zip(moon_vel.iter())
                    .all(|(a, b)| a == b))
            {
                break;
            }
        }

        println!("Dimension {} looped in {} steps", dim, counter);
        counts.push(counter);
    }

    let lcm = integer::lcm(integer::lcm(counts[0] as i64, counts[1] as i64), counts[2] as i64);
    println!("LCM: {}", lcm);
}

#[derive(Debug, Clone)]
struct Moon {
    position: Vector3,
    velocity: Vector3,
    acceleration: Vector3,
    id: usize,
    potential_energy: f64,
    kinetic_energy: f64,
}

impl Moon {
    fn new(position: Vector3, id: usize) -> Moon {
        Moon {
            position,
            velocity: Vector3::create_zero(),
            acceleration: Vector3::create_zero(),
            id,
            potential_energy: 0.,
            kinetic_energy: 0.,
        }
    }

    fn tick(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration.zero();
    }

    fn calculate_gravity(&mut self, other_moon: &Moon) {
        self.acceleration.x += match self.position.x.partial_cmp(&other_moon.position.x) {
            Some(Ordering::Less) => 1.0,
            Some(Ordering::Equal) => 0.0,
            Some(Ordering::Greater) => -1.0,
            None => panic!("Should never reach this"),
        };

        self.acceleration.y += match self.position.y.partial_cmp(&other_moon.position.y) {
            Some(Ordering::Less) => 1.0,
            Some(Ordering::Equal) => 0.0,
            Some(Ordering::Greater) => -1.0,
            None => panic!("Should never reach this"),
        };

        self.acceleration.z += match self.position.z.partial_cmp(&other_moon.position.z) {
            Some(Ordering::Less) => 1.0,
            Some(Ordering::Equal) => 0.0,
            Some(Ordering::Greater) => -1.0,
            None => panic!("Should never reach this"),
        };

        // println!("Acceleration: {}", self.acceleration);
    }

    fn calculate_energy(&mut self) {
        self.potential_energy =
            self.position.x.abs() + self.position.y.abs() + self.position.z.abs();
        self.kinetic_energy = self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs();
    }
}

impl PartialEq for Moon {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Moon {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Eq for Moon {}

#[derive(Debug, Clone, Copy)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    fn create_zero() -> Vector3 {
        Vector3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    fn add(&mut self, other: Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    fn sub(&mut self, other: Vector3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }

    fn zero(&mut self) {
        self.x = 0.;
        self.y = 0.;
        self.z = 0.;
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < 1e-5
            && (self.y - other.y).abs() < 1e-5
            && (self.z - other.z).abs() < 1e-5
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x={}, y={}, z={})", self.x, self.y, self.z)
    }
}
