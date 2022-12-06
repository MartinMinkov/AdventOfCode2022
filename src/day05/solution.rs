use std::collections::VecDeque;
use std::fs::read_to_string;

pub fn solve() {
    let path = "./src/day05/large-input1.txt";
    let input = read_to_string(path).expect("Should have been able to read the file");

    let mut stacks1: VecDeque<Vec<String>> = VecDeque::new();
    let mut stacks2: VecDeque<Vec<String>> = VecDeque::new();

    init_stacks(
        &input,
        get_number_of_stacks(&input),
        &mut stacks1,
        &mut stacks2,
    );
    let move_input = get_move_input(&input);

    part1(&move_input, &mut stacks1);
    part2(&move_input, &mut stacks2);
}

fn part1(input: &Vec<&str>, stacks: &mut VecDeque<Vec<String>>) {
    for line in input {
        let numbers: Vec<&str> = line
            .split(|c: char| !c.is_numeric())
            .filter(|s| !s.is_empty())
            .collect();

        let number_of_pops = numbers[0].parse::<usize>().unwrap();
        let src = numbers[1].parse::<usize>().unwrap() - 1;
        let dst = numbers[2].parse::<usize>().unwrap() - 1;

        let n = stacks[src].len() - number_of_pops;
        let popped = stacks[src].drain(n..).collect::<VecDeque<_>>();
        stacks[dst].extend(popped);
    }

    let mut result = String::new();
    for stack in stacks {
        if stack.len() > 0 {
            result.push_str(stack.last().unwrap());
        } else {
            result.push_str("");
        }
    }
    println!("Part 1: {}", result);
}

fn part2(input: &Vec<&str>, stacks: &mut VecDeque<Vec<String>>) {
    for line in input {
        let numbers: Vec<&str> = line
            .split(|c: char| !c.is_numeric())
            .filter(|s| !s.is_empty())
            .collect();

        let number_of_pops = numbers[0].parse::<usize>().unwrap();
        let src = numbers[1].parse::<usize>().unwrap() - 1;
        let dst = numbers[2].parse::<usize>().unwrap() - 1;

        let n = stacks[src].len() - number_of_pops;
        let popped = stacks[src].drain(n..).rev().collect::<VecDeque<_>>();
        stacks[dst].extend(popped);
    }

    let mut result = String::new();
    for stack in stacks {
        if stack.len() > 0 {
            result.push_str(stack.last().unwrap());
        } else {
            result.push_str("");
        }
    }
    println!("Part 2: {}", result);
}

fn get_number_of_stacks(input: &str) -> i32 {
    let mut lines: Vec<&str> = vec![];
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        lines.push(line);
    }
    lines[lines.len() - 1]
        .split("  ")
        .collect::<Vec<&str>>()
        .len() as i32
}

fn get_stack_input(input: &str) -> Vec<&str> {
    let mut lines: Vec<&str> = vec![];
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        lines.push(line);
    }
    lines.pop();
    lines.reverse();
    lines
}

fn get_move_input(input: &str) -> Vec<&str> {
    let mut lines: Vec<&str> = vec![];
    let mut found = false;
    for line in input.lines() {
        if line.is_empty() {
            found = true;
        }
        if found && !line.is_empty() {
            lines.push(line);
        }
    }
    lines
}

fn init_stacks(
    input: &str,
    number_of_stacks: i32,
    stack1: &mut VecDeque<Vec<String>>,
    stack2: &mut VecDeque<Vec<String>>,
) {
    for _ in 0..number_of_stacks {
        stack1.push_back(vec![]);
        stack2.push_back(vec![]);
    }

    let stack_input = get_stack_input(input);

    for line in stack_input {
        let mut iter = line.chars();
        let mut current_line = 0;

        while let Some(ch) = iter.next() {
            if ch == ' ' {
                iter.nth(2);
                current_line += 1;
                continue;
            } else if ch.is_alphabetic() {
                stack1[current_line].push(ch.to_string());
                stack2[current_line].push(ch.to_string());
                iter.nth(1);
                current_line += 1;
                continue;
            }
        }
    }
}
