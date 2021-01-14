use colored::*;
use ndarray::prelude::*;
use itertools::Itertools;

pub fn part1(input: String) {
    let nums: Vec<u32> = input.trim().chars().map(|c| { c.to_digit(10).expect("Failed to parse digit") }).collect();

    println!("Length: {}", nums.len());

    let columns = 25;
    let rows = 6;

    // let columns = 3;
    // let rows = 2;
    
    let layers = nums.len() / (rows * columns);

    println!("Expect {} layers", layers);

    let img = Array::from_shape_vec((layers, rows, columns), nums).expect("");

    println!("Constructed image array with dimensions {:?}", img.dim());

    let mut max_non_zero = 0;
    let mut max_layer = 0;
    let mut one_by_two = 0;

    for (l, a) in img.axis_iter(Axis(0)).enumerate() {
        // println!("{}", a);
        let count_non_zero = a.fold(0, |acc, &x| if x > 0 {acc + 1} else {acc});
        let count_one = a.fold(0, |acc, &x| if x == 1 {acc + 1} else {acc});
        let count_two = a.fold(0, |acc, &x| if x == 2 {acc + 1} else {acc});

        if count_non_zero > max_non_zero {
            max_non_zero = count_non_zero;
            max_layer = l;
            one_by_two = count_one * count_two;
        }
    }

    println!("Maximum non-zero is {}, layer {}, number(1) * number(2) = {}", max_non_zero, max_layer, one_by_two);
}

pub fn part2(input: String) {
    let nums: Vec<u32> = input.trim().chars().map(|c| { c.to_digit(10).expect("Failed to parse digit") }).collect();

    println!("Length: {}", nums.len());

    let columns = 25;
    let rows = 6;

    // let columns = 2;
    // let rows = 2;
    
    let layers = nums.len() / (rows * columns);

    println!("Expect {} layers", layers);

    let img = Array::from_shape_vec((layers, rows, columns), nums).expect("");

    println!("Constructed image array with dimensions {:?}", img.dim());

    let mut final_image = Array::from_elem((rows, columns), 2);

    for a in img.axis_iter(Axis(0)) {
        final_image.zip_mut_with(&a, |x1, &x2| {
            // println!("{}, {}", x1, x2);
            if *x1 == 2 {
                *x1 = x2;
            }
        });
    }

    for i in 0..final_image.nrows() {
        for j in 0..final_image.ncols() {
            if final_image[[i,j]] == 0 {
                print!("{}", "▄".black());
            }
            else {
                print!("{}", "▄".white());
            }
            // print!("{}", final_image[[i, j]]);
        }
        println!("");
    }
}
