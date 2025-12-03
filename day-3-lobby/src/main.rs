use std::io;
use std::io::Read;
use timer;

fn step1(lines: &[&str]) -> usize {
    let mut answer = 0;
    for line in lines {
        let mut high: usize = 0;
        let chars: Vec<char> = line.chars().collect();

        for (i, c) in chars.iter().enumerate() {
            for j in (i + 1)..chars.len() {
                let n = format!("{}{}", c, chars[j]).parse::<usize>().unwrap();
                if n > high {
                    high = n;
                }
            }
        }
        answer += high;
    }
    answer
}

fn step2(lines: &[&str]) -> usize {
    let mut answer = 0;
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let total_size = 12;

        let mut res = String::new();
        let mut start = 0;

        for _ in 0..total_size {
            let mut best = (0, '0');
            let remaining = total_size - res.len() - 1;
            let end = chars.len() - remaining;

            for i in start..end {
                if chars[i] > best.1 {
                    best = (i, chars[i]);
                }
            }

            res.push(best.1);
            start = best.0 + 1;
        }
        answer += res.parse::<usize>().unwrap();
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
