use std::io;
use std::collections::HashMap;
use std::io::Read;
use timer;


fn step1(lines: &[&str]) -> usize {
    let mut answer = 0;
    let grid: Vec<Vec<&str>> = lines.into_iter().map(|f| f.trim().split_whitespace().collect()).collect();
    let max_x = grid[0].len();
    let max_y = grid.len() - 1;

    for x in (0..max_x).rev() {
        let op = grid[max_y][x];
        let mut r = 0;
        for y in 0..max_y {
            let n = grid[y][x].parse::<usize>().unwrap();
            if y == 0 {
                r = n;
            } else {
                match op {
                    "*" => r *= n,
                    "+" => r += n,
                    _ => panic!("Operator not handled")
                }
            }
        }
        answer += r;
    }

    answer
}

fn add_map(map: HashMap<i32, Vec<char>>, op: char) -> usize {
    let mut r: usize = if op == '*' { 1 } else { 0 };

    for v in map.values() {
        let n = v.iter().collect::<String>().parse::<usize>().unwrap();
        match op {
            '+' => r += n,
            '*' => r *= n,
            _ => panic!("Invalid op")
        }
    }

    r
}

fn step2(lines: &[&str]) -> usize {
    let mut answer = 0;
    let grid: Vec<Vec<char>> = lines.into_iter().map(|f| f.chars().collect()).collect();
    let max_x = grid[0].len();
    let max_y = grid.len() - 1;

    let mut numbers: HashMap<i32, Vec<char>> = HashMap::new();
    let mut n_index = 0;

    for x in (0..max_x).rev() {
        for y in 0..max_y + 1 {
            let c = grid[y][x];
            match c {
                c if c.is_ascii_digit() => {
                    numbers.entry(n_index).and_modify(|v| v.push(c)).or_insert(vec![c]);
                }
                '+' | '*' => {
                    answer += add_map(numbers, c);
                    numbers = HashMap::new();
                    n_index = 0;
                }
                ' ' => (),
                _ => panic!("Incorrect input")
            }
        }
        n_index += 1;
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

