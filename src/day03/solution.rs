use std::fs::read_to_string;

pub fn solve() {
    let path = "./src/day03/input.txt";
    let input = read_to_string(path).expect("Should have been able to read the file");

    let ans1 = part1(input.clone());
    println!("Part 1: {}", ans1);

    let ans2 = part2(input);
    println!("Part 2: {}", ans2);
}

fn part1(input: String) -> i32 {
    let mut sum = 0;
    for line in input.split("\n") {
        let length = line.len();
        let (left, right) = line.split_at(length / 2);
        let duplicate_type =
            find_duplicate_type(left, right).expect("Could not find duplicate type");
        sum += char_to_priority(duplicate_type);
    }
    sum
}

fn part2(input: String) -> i32 {
    let mut sum = 0;
    let mut groups: Vec<String> = Vec::new();

    for (i, line) in input.split("\n").into_iter().enumerate() {
        if i % 3 == 2 {
            groups.push(line.to_string());
            let duplicate_type = find_duplicate_type2(&groups[0], &groups[1], &groups[2])
                .expect("Could not find duplicate type");
            sum += char_to_priority(duplicate_type);
            groups.clear();
        } else {
            groups.push(line.to_string());
        }
    }
    sum
}

fn find_duplicate_type(s1: &str, s2: &str) -> Option<char> {
    for c in s1.chars() {
        if s2.contains(c) {
            return Some(c);
        }
    }
    return None;
}

fn find_duplicate_type2(s1: &str, s2: &str, s3: &str) -> Option<char> {
    for c in s1.chars() {
        if s2.contains(c) && s3.contains(c) {
            return Some(c);
        }
    }
    return None;
}

fn char_to_priority(c: char) -> i32 {
    let ascii_code = c as i32;
    if ascii_code >= 65 && ascii_code <= 90 {
        return ascii_code - 65 + 27;
    }
    return ascii_code - 97 + 1;
}
