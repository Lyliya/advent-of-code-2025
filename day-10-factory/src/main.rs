// Based on https://github.com/MizardX/AdventOfCode_2025/blob/main/src/day_10.rs answer for using microlp in step 2

use std::io;
use std::io::Read;
use std::collections::{HashSet, VecDeque};
use microlp::{LinearExpr, OptimizationDirection, Problem, ComparisonOp};
use timer;

fn parse_goal(goal: &str) -> u16 {
    let mut bin = 0;

    for (i, c) in goal[1..goal.len() - 1].chars().enumerate() {
        if c == '#' {
            bin |= 1 << i;
        }
    }

    bin
}

fn parse_button(button: &str) -> u16 {
    let mut bin = 0;
    for num in button[1..button.len() - 1].split(',').collect::<Vec<&str>>() {
        if let Ok(bit) = num.parse::<u32>() {
            bin |= 1 << bit;
        }
    }

    bin
}

fn bfs(goal: u16, button: &Vec<u16>) -> u16 {
    let mut queue: VecDeque<(u16, u16)> = VecDeque::new();
    queue.push_back((0, 0));

    let mut visited = HashSet::new();
    visited.insert(0);

    while let Some((curr, steps)) = queue.pop_front() {
        if curr == goal {
            return steps;
        }

        for mask in button {
            let next = curr ^ mask;
            if visited.insert(next) {
                queue.push_back((next, steps + 1));
            }
        }
    }

    0
}

fn step1(lines: &[&str]) -> u16 {
    let mut answer = 0;

    for line in lines {
        let split: Vec<&str> = line.trim().split(" ").collect();
        let goal = parse_goal(split[0]);
        let button: Vec<_> = split[1..split.len() - 1].iter().map(|v| parse_button(&v)).collect();

        answer += bfs(goal, &button);
    }
    answer
}

fn solve_lp(button: &Vec<u16>, goal: &Vec<u16>) -> u32 {
    let mut problem = Problem::new(OptimizationDirection::Minimize);
    let max = goal.iter().max().unwrap();
    let button_vars = button
        .iter()
        .map(|_| problem.add_integer_var(1.0, (0, i32::from(*max))))
        .collect::<Vec<_>>();

    for (light_ix, &trg) in goal.iter().enumerate() {
        let mut expr = LinearExpr::empty();
        for (btn_mask, &btn_var) in button.iter().zip(&button_vars) {
            if btn_mask & (1 << light_ix) != 0 {
                expr.add(btn_var, 1.0);
            }
        }
        problem.add_constraint(expr, ComparisonOp::Eq, f64::from(trg));
    }
    problem.solve().unwrap().objective().round() as u32
}

fn step2(lines: &[&str]) -> u32 {
    let mut answer = 0;

    for line in lines {
        let split: Vec<&str> = line.trim().split(" ").collect();
        let goal_str = split[split.len() - 1];
        let goal: Vec<u16> = goal_str[1..goal_str.len() - 1].split(',').filter_map(|v| v.parse::<u16>().ok()).collect();
        let buttons: Vec<u16> = split[1..split.len() - 1].iter().map(|v| parse_button(&v)).collect();

        answer += solve_lp(&buttons, &goal);
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
    
    let (step2_answer, step2_time) = timer::measure(|| step2(&lines));
    println!("Step 2 answer: {}, in {:?}", step2_answer, step2_time);
}