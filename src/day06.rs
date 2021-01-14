use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use itertools::Itertools;

pub fn part1(input: String) {
    // Build map
    let mut map = HashMap::new();

    for line in input.lines() {
        // println!("{}", line);
        let (p, o) = line.split(")").collect_tuple().unwrap();
        map.insert(o, p);
    }

    let mut direct = 0;
    let mut indirect = 0;

    for i in &map {
        // println!("{:?}", i);
        direct += 1;

        let mut obj = *i.1;

        loop {
            match obj {
                "COM" => break,
                _ => {
                    indirect += 1;
                    obj = map[obj];
                },
            }
        }
    }

    println!("Direct: {}, Indirect: {}", direct, indirect);
    println!("Total: {}", direct + indirect);
}

pub fn part2(input: String) {
    // Build map
    let mut map = HashMap::new();

    for line in input.lines() {
        // println!("{}", line);
        let (p, o) = line.split(")").collect_tuple().unwrap();
        map.insert(o, p);
    }

    // Build orbit trees for santa and you
    let you_list = build_orbit_list("YOU", &map);
    let santa_list = build_orbit_list("SAN", &map);

    let santa_hash: HashSet<&str> = HashSet::from_iter(santa_list.iter().cloned());

    let mut closest_common_point = "";
    let mut distance = 0;

    for (i, en) in you_list.iter().enumerate() {
        if santa_hash.contains(en) {
            println!("Found closest common point: {}", en);
            closest_common_point = en;
            distance = i;
            break;
        }
    }

    println!("Common point: {}, {} jumps away", closest_common_point, distance);

    for (i, en) in santa_list.iter().enumerate() {
        if *en == closest_common_point {
            println!("{} hops from common point to Santa", i);
            distance += i;
            break;
        }
    }

    println!("Total number of hops is {}", distance);
}

fn build_orbit_list<'a>(mut start: &'a str, orbital_map: &'a HashMap<&str,&str>) -> Vec<&'a str> {
    let mut ret = Vec::new();

    loop {
        match start {
            "COM" => {
                break;
            },
            _ => {
                start = orbital_map[start];
                ret.push(start);
            }
        }
    }

    ret
}
