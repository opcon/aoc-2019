use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
    steps: i32
}

impl Point {
    fn distance(&self, p2: &Point) -> i32 {
        (self.x - p2.x).abs() + (self.y - p2.y).abs()
    }
}

impl fmt::Display for Point {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

pub fn part1(input: String) {

    let mut line_points = Vec::new();

    // iterate over our wires
    for l in input.lines() {
        let mut current_line_points = HashSet::new();
        current_line_points.insert(Point { x: 0, y: 0, steps: 0 });

        let mut current_point = Point { x: 0, y: 0, steps: 0 };
        // iterate over our coordinates
        for sp in l.split(",") {
            let (dir, count_str) = sp.trim().split_at(1);
            let count_num = count_str.parse::<i32>().unwrap();
            //println!("Direction: {}, Count: {}", dir, count_num);
            
            // println!("Moving {} by {}, current point is {}", dir, count_num, current_point);

            match dir {
                "U" => {
                    for i in 1..=count_num {
                        // println!("{}", i);
                        current_line_points.insert(Point { x: current_point.x, y: current_point.y + i, steps: 0 });
                    }
                    current_point.y += count_num;
                },
                "D" => {
                    for i in (-count_num..=-1).rev() {
                        // println!("{}", i);
                        current_line_points.insert(Point { x: current_point.x, y: current_point.y + i, steps: 0 });
                    }
                    current_point.y -= count_num;
                },
                "L" => {
                    for i in (-count_num..=-1).rev() {
                        // println!("{}", i);
                        current_line_points.insert(Point { x: current_point.x + i, y: current_point.y, steps: 0 });
                    }
                    current_point.x -= count_num;
                },
                "R" => {
                    for i in 1..=count_num {
                        // println!("{}", i);
                        current_line_points.insert(Point { x: current_point.x + i, y: current_point.y, steps: 0 });
                    }
                    current_point.x += count_num;
                },
                _ => println!("Invalid direction found: {}!", dir),
            }
            // println!("New point is {}", current_point);
        }
        // println!("Length of line points is {}", current_line_points.len());
        // println!("Points are:");
        // for p in current_line_points.iter() {
        //     println!("{}", p);
        // }
        line_points.push(current_line_points);
    }

    let origin = Point { x: 0, y: 0, steps: 0 };
    let mut min_dist = std::i32::MAX;

    for i in line_points[0].intersection(&line_points[1]) {
        let dist = i.distance(&origin);
        println!("Intersection at {}, distance = {}", i, dist);
        if dist > 0 && dist < min_dist {
            min_dist = dist;
        }
    }

    println!("Minimum distance: {}", min_dist);
}

pub fn part2(input: String) {

    let mut line_points = Vec::new();

    // iterate over our wires
    for l in input.lines() {
        let mut current_line_points = HashSet::new();
        current_line_points.insert(Point { x: 0, y: 0, steps: 0 });

        let mut current_point = Point { x: 0, y: 0, steps: 0 };
        // iterate over our coordinates
        for sp in l.split(",") {
            let (dir, count_str) = sp.trim().split_at(1);
            let count_num = count_str.parse::<i32>().unwrap();
            //println!("Direction: {}, Count: {}", dir, count_num);
            
            // println!("Moving {} by {}, current point is {}", dir, count_num, current_point);

            match dir {
                "U" => {
                    for i in 1..=count_num {
                        // println!("{}", i);
                        current_line_points.insert(Point { 
                            x: current_point.x,
                            y: current_point.y + i,
                            steps: current_point.steps + i.abs(),
                        });
                        // current_line_points.insert(Point { x: current_point.x, y: current_point.y + i, steps: 0 });
                    }
                    current_point.y += count_num;
                    current_point.steps += count_num;
                },
                "D" => {
                    for i in (-count_num..=-1).rev() {
                        // println!("{}", i);
                        current_line_points.insert(Point { 
                            x: current_point.x,
                            y: current_point.y + i,
                            steps: current_point.steps + i.abs(),
                        });
                        // current_line_points.insert(Point { x: current_point.x, y: current_point.y + i, steps: 0 });
                    }
                    current_point.y -= count_num;
                    current_point.steps += count_num;
                },
                "L" => {
                    for i in (-count_num..=-1).rev() {
                        // println!("{}", i);
                        current_line_points.insert(Point { 
                            x: current_point.x + i,
                            y: current_point.y,
                            steps: current_point.steps + i.abs(),
                        });
                        // current_line_points.insert(Point { x: current_point.x + i, y: current_point.y, steps: 0 });
                    }
                    current_point.x -= count_num;
                    current_point.steps += count_num;
                },
                "R" => {
                    for i in 1..=count_num {
                        // println!("{}", i);
                        current_line_points.insert(Point { 
                            x: current_point.x + i,
                            y: current_point.y,
                            steps: current_point.steps + i.abs(),
                        });
                        // current_line_points.insert(Point { x: current_point.x + i, y: current_point.y, steps: 0 });
                    }
                    current_point.x += count_num;
                    current_point.steps += count_num;
                },
                _ => println!("Invalid direction found: {}!", dir),
            }
            // println!("New point is {}", current_point);
        }
        println!("Length of line points is {}", current_line_points.len());
        // println!("Points are:");
        // for p in current_line_points.iter() {
        //     println!("{}", p);
        // }
        line_points.push(current_line_points);
    }

    let origin = Point { x: 0, y: 0, steps: 0 };
    let mut min_steps = std::i32::MAX;

    for i in line_points[0].intersection(&line_points[1]) {
        let p1 = line_points[0].get(i).unwrap();
        let p2 = line_points[1].get(i).unwrap();
        let steps = p1.steps + p2.steps;
        println!("Intersection at {}, steps = {}", i, steps);
        if steps > 0 && steps < min_steps {
            min_steps = steps;
        }
    }

    println!("Minimum steps: {}", min_steps);
}
