use core::str;
use std::io;
use std::io::Read;
use timer;

fn step1(lines: Vec<&str>) -> usize {
    let mut answer = 0;
    let mut pos = 50;

    for line in lines {
        let dir = line.chars().next().unwrap();
        let n = line[1..].parse::<u32>().unwrap();

        for _i in 0..n {
            if dir == 'L' {
                pos -= 1
            }
            if dir == 'R' {
                pos += 1
            }
            if pos == -1 {
                pos = 99
            }
            if pos == 100 {
                pos = 0
            }
        }
        if pos == 0 {
            answer += 1
        }
    }
    answer
}

fn step2(lines: Vec<&str>) -> usize {
    let mut answer = 0;
    let mut pos = 50;

    for line in lines {
        let dir = line.chars().next().unwrap();
        let n = line[1..].parse::<u32>().unwrap();

        for _i in 0..n {
            if dir == 'L' {
                pos -= 1
            }
            if dir == 'R' {
                pos += 1
            }
            if pos == -1 {
                pos = 99
            }
            if pos == 100 {
                pos = 0
            }
            if pos == 0 {
                answer += 1
            }
        }
    }
    answer
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");
    let lines: Vec<&str> = input.lines().collect();

    let (step1_answer, step1_time) = timer::measure(|| step1(lines.clone()));
    let (step2_answer, step2_time) = timer::measure(|| step2(lines.clone()));

    println!("Step 1 answer: {}, in {:?}", step1_answer, step1_time);
    println!("Step 2 answer: {}, in {:?}", step2_answer, step2_time);
}
