use std::fs::read_to_string;

pub fn solve() {
    let path = "./src/day06/input.txt";
    let input = read_to_string(path).expect("Should have been able to read the file");
    let ans1 = part1(&input);
    let ans2 = part2(&input);
    println!("Part 1: {}", ans1);
    println!("Part 2: {}", ans2);
}

fn part1(input: &str) -> i32 {
    let mut bytes = [0u8; 4];
    let data = input.as_bytes();
    let mut curr = 0;
    let mut i = 0;

    while i <= data.len() {
        let c = &data[i];
        if curr == 4 {
            return i as i32;
        }
        if contains1(&bytes, &c) {
            i -= curr - 1;
            curr = 1;
            bytes = [0u8; 4];
            bytes[0] = data[i];
        } else {
            bytes[curr] = *c;
            curr += 1;
        }
        i += 1
    }

    return -1;
}

fn part2(input: &str) -> i32 {
    let mut bytes = [0u8; 14];
    let data = input.as_bytes();
    let mut curr = 0;
    let mut i = 0;

    while i <= data.len() {
        let c = &data[i];
        if curr == 14 {
            return i as i32;
        }
        if contains2(&bytes, &c) {
            i -= curr - 1;
            curr = 1;
            bytes = [0u8; 14];
            bytes[0] = data[i];
        } else {
            bytes[curr] = *c;
            curr += 1;
        }
        i += 1
    }
    return -1;
}

fn contains1(b: &[u8; 4], curr: &u8) -> bool {
    for i in 0..4 {
        if b[i] == *curr {
            return true;
        }
    }
    return false;
}

fn contains2(b: &[u8; 14], curr: &u8) -> bool {
    for i in 0..14 {
        if b[i] == *curr {
            return true;
        }
    }
    return false;
}
