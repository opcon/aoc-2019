use colored::*;
use itertools::Itertools;
use ndarray::prelude::*;
use ndarray::{Data, ScalarOperand};
use std::fmt;
use std::cmp;
use num;
use std::collections::HashSet;
use ordered_float::OrderedFloat;

pub fn part1(input: String) {
    println!("Day 10 Part 1:");

    let mut width = input.lines().count();
    let mut height = input
        .lines()
        .next()
        .expect("Error reading input")
        .trim()
        .len();

    println!("Grid is {}x{}", width, height);

    let mut asteroid_positions = Vec::new();
    let mut grid = Array::zeros((height, width));

    for (j, l) in input.lines().enumerate() {
        for (i, n) in l.chars().enumerate() {
            println!("{}", n);
            match n {
                '.' => grid[[j, i]] = 0,
                '#' => {
                    grid[[j, i]] = 1;
                    asteroid_positions.push(Point { x: i as f64, y: j as f64 });
                }
                _ => println!("Invalid character encountered in map"),
            }
        }
    }

    print_grid(&grid);

    let mut max_ast_count = 0;
    let mut max_pos = Point { x: 0., y: 0. };

    for e in asteroid_positions.iter() {
        let mut ast_count = 0;
        let mut ast_dist_pos = Vec::new();
        let mut visited_angles: Vec<f64> = Vec::new();
        // println!("Checking {}", e);
        let start = e.clone();

        for other in asteroid_positions.iter() {
            if other == e {
                continue;
            }
            let distance = e.distance(other);
            let angle = (e.y - other.y).atan2(other.x - e.x) * 180. / std::f64::consts::PI;
            // println!("distance to {} is {}, angle is {}", other, distance, angle);
            ast_dist_pos.push((distance, angle));
        }

        ast_dist_pos.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        for (dist, ang) in ast_dist_pos.iter() {
            if visited_angles.iter().any(|a| (a - ang).abs() < 1e-5 ) {
                // println!("Can not see asteroid at (r,theta) = ({}, {})", dist, ang);
                continue;
            }

            // println!("Can see asteroid at (r,theta) = ({}, {})", dist, ang);
            ast_count += 1;
            visited_angles.push(*ang);
        }

        // println!("Count: {}", ast_count);

        if ast_count > max_ast_count {
            max_ast_count = ast_count;
            max_pos = e.clone();
        }

        // println!("{:?}", ast_dist_pos);

        // for ang in Array::range(0., 2.0 * std::f64::consts::PI, std::f64::consts::PI / 3600.).iter() {
        //     // println!("{}", ang);
        //     let radii = Array::range(0., max_radius as f64, 0.1);
        //     let x_vals = radii.iter().map(|r| (*r as f64) * (*ang as f64).cos() + e.x as f64).skip(1);
        //     let y_vals = radii.iter().map(|r| (*r as f64) * (*ang as f64).sin() + e.y as f64).skip(1);

        //     for (x,y) in x_vals.zip(y_vals) {
        //         let x_round = x.round();
        //         let y_round = y.round();

        //         if (x_round < 0.0 || x_round >= width as f64) || (y_round < 0.0 || y_round >= height as f64) {
        //             continue;
        //         }
        //         if (x_round - x).abs() < 1e-5 && (y_round - y).abs() < 1e-5 {
        //             println!("Integer position found: x: {}, y: {}", x_round, y_round);
        //             if grid[[y_round as usize, x_round as usize]] == 1 {
        //                 println!("Asteroid seen at: (x,y) = ({}, {})", x_round as i32, y_round as i32);
        //             }
        //         }
        //     }
        // }
    }

    println!("Max count is {} at {}", max_ast_count, max_pos);
}

pub fn part2(input: String) {
    println!("Day 10 Part 2:");

    let mut width = input.lines().count();
    let mut height = input
        .lines()
        .next()
        .expect("Error reading input")
        .trim()
        .len();

    println!("Grid is {}x{}", width, height);

    let mut asteroid_positions = Vec::new();
    let mut grid = Array::zeros((height, width));

    for (j, l) in input.lines().enumerate() {
        for (i, n) in l.chars().enumerate() {
            println!("{}", n);
            match n {
                '.' => grid[[j, i]] = 0,
                '#' => {
                    grid[[j, i]] = 1;
                    asteroid_positions.push(Point { x: i as f64, y: j as f64 });
                }
                _ => println!("Invalid character encountered in map"),
            }
        }
    }

    print_grid(&grid);

    let mut max_ast_count = 0;
    let mut max_pos = Point { x: 0., y: 0. };

    for e in asteroid_positions.iter() {
        let mut ast_count = 0;
        let mut ast_dist_pos = Vec::new();
        let mut visited_angles: Vec<f64> = Vec::new();
        // println!("Checking {}", e);
        let start = e.clone();

        for other in asteroid_positions.iter() {
            if other == e {
                continue;
            }
            let distance = e.distance(other);
            let angle = (e.y - other.y).atan2(other.x - e.x) * 180. / std::f64::consts::PI;
            // println!("distance to {} is {}, angle is {}", other, distance, angle);
            ast_dist_pos.push((distance, angle));
        }

        ast_dist_pos.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        for (dist, ang) in ast_dist_pos.iter() {
            if visited_angles.iter().any(|a| (a - ang).abs() < 1e-5 ) {
                // println!("Can not see asteroid at (r,theta) = ({}, {})", dist, ang);
                continue;
            }

            // println!("Can see asteroid at (r,theta) = ({}, {})", dist, ang);
            ast_count += 1;
            visited_angles.push(*ang);
        }

        // println!("Count: {}", ast_count);

        if ast_count > max_ast_count {
            max_ast_count = ast_count;
            max_pos = e.clone();
        }
    }

    let mut asteroids_to_vaporise: Vec<Asteroid> = Vec::new();
    let outpost_pos = max_pos;

    println!("Outpost pos is {:?}", outpost_pos);

    for other in asteroid_positions.iter() {
        if *other == outpost_pos {
            continue;
        }

        let mut ang = (360. - (outpost_pos.y - other.y).atan2(other.x - outpost_pos.x) * 180. / std::f64::consts::PI + 90.) % 360.;
        if ang < 0. {
            ang += 360.;
        }
        asteroids_to_vaporise.push( Asteroid { x: other.x, y: other.y, r: outpost_pos.distance(other), ang } );
    }

    asteroids_to_vaporise.sort_by_key(|a| (OrderedFloat(a.ang), OrderedFloat(a.r)));

    println!("Length: {}", asteroids_to_vaporise.len());

    let start_index = asteroids_to_vaporise.iter().position(|ast| ast.ang == 0.).unwrap();
    println!("First asteroid to be vaporised found at index {}, {:?}", start_index, asteroids_to_vaporise[start_index]);

    let mut visited_angles: Vec<f64> = Vec::new();
    let mut asteroids_to_remove = Vec::new();

    for a in asteroids_to_vaporise.iter() {
        if visited_angles.iter().any(|b| (a.ang - b).abs() < 1e-5) {
            continue;
        }
        asteroids_to_remove.push(a);
        visited_angles.push(a.ang);
    }

    println!("Found {} asteroids to remove in first pass", asteroids_to_remove.len());

    println!("1st asteroid is at {},{}", asteroids_to_remove[0].x, asteroids_to_remove[0].y);
    println!("2nd asteroid is at {},{}", asteroids_to_remove[1].x, asteroids_to_remove[1].y);
    println!("3rd asteroid is at {},{}", asteroids_to_remove[2].x, asteroids_to_remove[2].y);
    println!("200th asteroid is {:?}, 100x+y = {}", asteroids_to_remove[199], asteroids_to_remove[199].x*100. + asteroids_to_remove[199].y);

    // println!("{:?}", asteroids_to_vaporise);

}

fn print_grid(grid: &Array2<i32>) {
    for j in 0..grid.nrows() {
        for i in 0..grid.ncols() {
            if grid[[j, i]] == 0 {
                print!("{}", "▄".blue());
            } else {
                print!("{}", "▄".red());
            }
            // print!("{}", final_image[[i, j]]);
        }
        println!("");
    }
}

#[derive(PartialEq, Hash, Eq, Debug, Clone)]
struct Point<T> {
    x: T,
    y: T,
}

// impl<T: num::Float> Point<T> {
//     fn distance(&self, p2: &Point<T>) -> T {
//         (self.x - p2.x).abs() + (self.y - p2.y).abs()
//     }
// }

impl<T: num::Float> Point<T> {
    fn distance(&self, p2: &Point<T>) -> T {
        ((self.x - p2.x).powi(2) + (self.y - p2.y).powi(2)).sqrt()
    }

    fn manhattan_distance(&self, p2: &Point<T>) -> T {
        (self.x - p2.x).abs() + (self.y - p2.y).abs()
    }
}

impl<T: fmt::Display> fmt::Display for Point<T> {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
struct Asteroid {
    x: f64,
    y: f64,
    r: f64,
    ang: f64,
}
