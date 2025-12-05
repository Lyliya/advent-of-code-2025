use std::io;
use std::io::Read;
use timer;

fn is_fresh(id: usize, ranges: &Vec<(usize, usize)>) -> bool {
    for range in ranges.iter() {
        if id >= range.0 && id <= range.1 {
            return true;
        }
    }
    return false;
}

fn step1(lines: &[&str]) -> usize {
    let mut answer = 0;
    let mut ranges: Vec<(usize, usize)> = vec![];

    for line in lines {
        if line.contains('-') {
            let (a, b) = line.split_once('-').unwrap();
            ranges.push((a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()));
        } else if let Ok(value) = line.parse::<usize>() {
            if is_fresh(value, &ranges) { answer += 1 }
        }
    }

    answer
}

fn merge(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    ranges.sort_by_key(|r| r.0);

    let mut result = Vec::new();
    let mut current = ranges[0];

    for &range in &ranges[1..] {
        if range.0 <= current.1 { 
            current.1 = current.1.max(range.1);
        } else {
            result.push(current);
            current = range;
        }
    }

    result.push(current);
    result
}

fn step2(lines: &[&str]) -> usize {
    let mut ranges: Vec<(usize, usize)> = vec![];
    let mut answer: usize = 0;

    for line in lines {
        if line.contains('-') {
            let (a, b) = line.split_once('-').unwrap();
            ranges.push((a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()));
        }
    }

    ranges = merge(ranges);

    for range in ranges {
        answer += (range.1 - range.0) + 1
    }

    answer
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");
    let lines: Vec<&str> = input.lines().collect();

    let (step1_answer, step1_time) = timer::measure(|| step1(&lines));
    let (step2_answer, step2_time) = timer::measure(|| step2(&lines));

    println!("Step 1 answer: {}, in {:?}", step1_answer, step1_time);
    println!("Step 2 answer: {}, in {:?}", step2_answer, step2_time);
}

