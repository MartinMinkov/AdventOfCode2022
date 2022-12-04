use std::fs::read_to_string;

pub fn solve() {
    let path = "./src/day04/input.txt";
    let input = read_to_string(path).expect("Should have been able to read the file");

    let ans1 = part1(input.clone());
    println!("Part 1: {}", ans1);

    let ans2 = part2(input.clone());
    println!("Part 2: {}", ans2);
}

fn part1(input: String) -> i32 {
    let mut pairs = 0;
    for line in input.lines() {
        let v: Vec<&str> = line.split(",").collect();

        let x: Vec<&str> = v[0].split("-").collect();
        let y: Vec<&str> = v[1].split("-").collect();

        let x1 = x[0].parse::<i32>().unwrap();
        let y1 = x[1].parse::<i32>().unwrap();

        let x2 = y[0].parse::<i32>().unwrap();
        let y2 = y[1].parse::<i32>().unwrap();

        if x1 <= x2 && y1 >= y2 {
            pairs += 1;
        } else if x2 <= x1 && y2 >= y1 {
            pairs += 1
        }
    }
    pairs
}

fn part2(input: String) -> i32 {
    let mut pairs = 0;
    for line in input.lines() {
        let v: Vec<&str> = line.split(",").collect();

        let x: Vec<&str> = v[0].split("-").collect();
        let y: Vec<&str> = v[1].split("-").collect();

        let x1 = x[0].parse::<i32>().unwrap();
        let y1 = x[1].parse::<i32>().unwrap();

        let x2 = y[0].parse::<i32>().unwrap();
        let y2 = y[1].parse::<i32>().unwrap();

        if y1 >= x2 && x1 <= y2 {
            pairs += 1;
        } else if x1 <= x2 && y1 >= y2 {
            pairs += 1
        }
    }
    pairs
}
