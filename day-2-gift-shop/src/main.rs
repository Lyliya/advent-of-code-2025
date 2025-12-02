use std::io;
use std::io::Read;

fn step1(lines: Vec<&str>) {
    let ranges = lines[0].split(",");
    let mut answer: usize = 0;
    for range in ranges {
        let parts: Vec<&str> = range.split("-").collect();
        if parts.len() != 2 {
            return;
        }
        let first: usize = parts[0].parse::<usize>().unwrap();
        let last: usize = parts[1].parse::<usize>().unwrap();
        for i in first..last + 1 {
            let str = i.to_string();
            if str.len() % 2 == 0 {
                let (a, b) = str.split_at(str.len() / 2);
                if a == b { answer += i };
            }
            
        }
    }
    println!("Step 1 answer: {}", answer)
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

fn step2(lines: Vec<&str>) {
    let ranges = lines[0].split(",");
    let mut answer: usize = 0;
    for range in ranges {
        let parts: Vec<&str> = range.split("-").collect();
        if parts.len() != 2 {
            return;
        }
        let first: usize = parts[0].parse::<usize>().unwrap();
        let last: usize = parts[1].parse::<usize>().unwrap();
        for i in first..last + 1 {
            let str = i.to_string();
            if check_perm(str) { answer += i }
        }
    }
    println!("Step 2 answer: {}", answer)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read stdin");
    let lines: Vec<&str> = input.lines().collect();
    step1(lines.clone());
    step2(lines.clone());
}
