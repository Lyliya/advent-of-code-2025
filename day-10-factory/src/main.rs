use std::io;
use std::io::Read;
use std::collections::HashSet;
use timer;

fn parse_button(button: &str) -> Vec<usize> {
    let nums: Vec<usize> = button.trim_start_matches('(')
        .trim_end_matches(')')
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();

    nums
}

fn merge(items: Vec<Vec<usize>>) -> Vec<usize> {
    let mut res = HashSet::new();

    for vec in items {
        for n  in vec {
            if res.contains(&n) {
                res.remove(&n);
            } else {
                res.insert(n);
            }
        }
    }

    let mut res: Vec<usize> = res.into_iter().collect();
    res.sort();
    res
}

fn combinations<T: Clone>(items: &[T], k: usize) -> Vec<Vec<T>> {
    if k == 0 { return vec![vec![]]; }
    if items.len() < k { return vec![]; }

    let mut res = vec![];
    let first = items[0].clone();

    for mut c in combinations(&items[1..], k - 1) {
        c.insert(0, first.clone());
        res.push(c);
    }

    res.extend(combinations(&items[1..], k));

    res
}

fn check_depth(goal: &Vec<usize>, button: &Vec<Vec<usize>>, depth: usize) -> bool {
    let c: Vec<Vec<usize>> = combinations(button, depth).into_iter().map(|v| merge(v)).collect();

    c.contains(goal)
}

fn found_combination_depth(goal: &Vec<usize>, button: &Vec<Vec<usize>>) -> usize {
    for i in 1..100 {
        if check_depth(&goal, &button, i) {
            return i;
        }
    }
    panic!("No combination found");
}

fn step1(lines: &[&str]) -> usize {
    let mut answer = 0;

    for line in lines {
        let split: Vec<&str> = line.trim().split(" ").collect();

        let goal: Vec<_> = split[0].trim_start_matches('[').trim_end_matches(']').chars().collect();
        let button: Vec<_> = split[1..split.len() - 1].iter().map(|v| parse_button(&v)).collect();

        let goal_nums: Vec<usize> = goal.iter().enumerate().filter_map(|(i, v)| if *v == '#' { Some(i)} else { None }).collect();

        answer += found_combination_depth(&goal_nums, &button);
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
    println!("Step 1 answer: {}, in {:?}", step1_answer, step1_time);
    
    // let (step2_answer, step2_time) = timer::measure(|| step2(&lines));
    // println!("Step 2 answer: {}, in {:?}", step2_answer, step2_time);
}