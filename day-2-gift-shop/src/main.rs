use std::io;
use std::io::Read;
use timer;

fn step1(lines: Vec<&str>) -> usize {
    let ranges = lines[0].split(",");
    let mut answer: usize = 0;
    for range in ranges {
        let parts: Vec<&str> = range.split("-").collect();
        if parts.len() != 2 {
            continue;
        }
        let first: usize = parts[0].parse::<usize>().unwrap();
        let last: usize = parts[1].parse::<usize>().unwrap();
        for i in first..last + 1 {
            let str = i.to_string();
            if str.len() % 2 == 0 {
                let (a, b) = str.split_at(str.len() / 2);
                if a == b {
                    answer += i
                };
            }
        }
    }
    return answer;
}

fn check_perm(str: String) -> bool {
    for len in 1..str.len() {
        let (a, _) = str.split_at(len);
        let r = a.repeat(str.len() / len);
        if r == str {
            return true;
        }
    }
    return false;
}

fn step2(lines: Vec<&str>) -> usize {
    let ranges = lines[0].split(",");
    let mut answer: usize = 0;
    for range in ranges {
        let parts: Vec<&str> = range.split("-").collect();
        if parts.len() != 2 {
            continue;
        }
        let first: usize = parts[0].parse::<usize>().unwrap();
        let last: usize = parts[1].parse::<usize>().unwrap();
        for i in first..last + 1 {
            let str = i.to_string();
            if check_perm(str) {
                answer += i
            }
        }
    }
    return answer;
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
