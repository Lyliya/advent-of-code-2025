use core::str;
use std::io;
use std::io::Read;

fn step1(lines: Vec<&str>) {
    let mut answer = 0;
    let mut pos = 50;

    for line in lines {
        let dir = line.chars().next().unwrap();
        let n = line[1..].parse::<u32>().unwrap();

        for _i in 0..n {
            if dir == 'L' {pos -= 1}
            if dir == 'R' {pos += 1}
            if pos == -1 {pos = 99}
            if pos == 100 {pos = 0}
        }
        if pos == 0 {answer += 1}
    }

    println!("Step 1 answer: {}", answer);
}

fn step2(lines: Vec<&str>) {
    let mut answer = 0;
    let mut pos = 50;

    for line in lines {
        let dir = line.chars().next().unwrap();
        let n = line[1..].parse::<u32>().unwrap();

        for _i in 0..n {
            if dir == 'L' {pos -= 1}
            if dir == 'R' {pos += 1}
            if pos == -1 {pos = 99}
            if pos == 100 {pos = 0}
            if pos == 0 {answer += 1}
        }
        
    }

    println!("Step 2 answer: {}", answer);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read stdin");
    let lines: Vec<&str> = input.lines().collect();

    step1(lines.clone());
    step2(lines.clone());
}
