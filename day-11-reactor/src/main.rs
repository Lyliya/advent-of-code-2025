use std::collections::HashMap;
use std::io;
use std::io::Read;
use timer;

fn parse_input(lines: &[&str]) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let split: Vec<_> = line.trim().split(':').collect();
        let main = split[0];
        for connection in split[1].trim().split_whitespace() {
            map.entry(main.to_string())
                .and_modify(|v| v.push(connection.to_string()))
                .or_insert(vec![connection.to_string()]);
        }
    }

    map
}

fn dfs(map: &HashMap<String, Vec<String>>, curr: &str, end: &str, memo: &mut HashMap<String, usize>) -> usize {
    if let Some(val) = memo.get(curr) {
        return *val;
    }
    
    if curr == end {
        memo.insert(curr.to_string(), 1);
        return 1;
    }

    if !map.contains_key(curr) {
        memo.insert(curr.to_string(), 0);
        return 0;
    }

    let mut total_path = 0;

    for next in map.get(curr).unwrap() {
        total_path += dfs(map, next, end, memo);
    }

    memo.insert(curr.to_string(), total_path);

    total_path
}

fn step1(lines: &[&str]) -> usize {
    let map = parse_input(lines);
    let mut memo: HashMap<String, usize> = HashMap::new();

    dfs(&map, "you", "out", &mut memo)
}

fn step2(lines: &[&str]) -> usize {
    let map = parse_input(lines);

    dfs(&map, "svr", "fft", &mut HashMap::new()) * dfs(&map, "fft", "dac", &mut HashMap::new()) * dfs(&map, "dac", "out", &mut HashMap::new()) + 
    dfs(&map, "svr", "dac", &mut HashMap::new()) * dfs(&map, "dac", "fft", &mut HashMap::new()) * dfs(&map, "fft", "out", &mut HashMap::new())
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