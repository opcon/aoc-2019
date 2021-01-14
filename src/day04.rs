pub fn part1() {
    let (start, end) = (402328, 864247);

    let mut count_valid = 0i32;

    for i in start..=end {
        let valid = is_valid(i);
        if valid {
            count_valid += 1;
            println!("Found valid password: {}", i);
        }
    }

    println!("Valid passwords: {}", count_valid);

    println!("111111: {}", is_valid(111111i32));
    println!("223450: {}", is_valid(223450i32));
    println!("123789: {}", is_valid(123789i32));
}

pub fn part2() {
    let (start, end) = (402328, 864247);

    let mut count_valid = 0i32;

    for i in start..=end {
        let valid = is_valid_p2(i);
        if valid {
            count_valid += 1;
            // println!("Found valid password: {}", i);
        }
    }

    println!("Valid passwords: {}", count_valid);

    println!("111111: {}", is_valid_p2(111111i32));
    println!("223450: {}", is_valid_p2(223450i32));
    println!("123789: {}", is_valid_p2(123789i32));

    println!("112233: {}", is_valid_p2(112233i32));
    println!("123444: {}", is_valid_p2(123444i32));
    println!("111122: {}", is_valid_p2(111122i32));

    println!("113456: {}", is_valid_p2(113456i32));
    println!("112355: {}", is_valid_p2(112355i32));
    println!("111233: {}", is_valid_p2(111233i32));
    println!("112333: {}", is_valid_p2(112333i32));
    println!("111114: {}", is_valid_p2(111114i32));
}

fn is_valid(n: i32) -> bool {
    let digits = number_to_vec(n);

    let mut valid = false;

    let mut prev_val = -1i32;
    for d in digits.iter() {
        if prev_val == -1 {
            prev_val = *d;
            continue;
        }

        if *d < prev_val {
            valid = false;
            break;
        }

        if *d == prev_val {
            valid = true;
        }
        prev_val = *d;
    }
    valid
}

fn is_valid_p2(n: i32) -> bool {
    let digits = number_to_vec(n);

    if has_decreasing_digits(&digits) {
        return false;
    }

    let repeated_digits = get_repeated_digits(&digits);

    for i in repeated_digits {
        if i.len() == 2 {
            return true;
        }
    }
    false
}

fn has_decreasing_digits(digits: &Vec<i32>) -> bool {
    let mut prev_val = -1i32;
    for d in digits.iter() {
        if prev_val == -1 {
            prev_val = *d;
            continue;
        }

        if *d < prev_val {
            return true;
        }
        prev_val = *d;
    }
    false
}

fn get_repeated_digits(digits: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut rep_dig: Vec<Vec<i32>> = Vec::new();
    let mut current_seq: Vec<i32> = Vec::new();

    for d in digits.iter() {
        if current_seq.len() == 0 {
            current_seq.push(*d);
            continue;
        }

        if d == current_seq.last().unwrap() {
            current_seq.push(*d);
            continue;
        } else {
            if current_seq.len() > 1 {
                rep_dig.push(current_seq);
            }
            current_seq = Vec::new();
            current_seq.push(*d);
        }
    }

    if current_seq.len() > 1 {
        rep_dig.push(current_seq);
    }
    rep_dig
}

fn number_to_vec(n: i32) -> Vec<i32> {
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
