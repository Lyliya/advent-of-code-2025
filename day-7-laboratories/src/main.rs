use std::collections::HashMap;
use std::io;
use std::io::Read;
use timer;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn find_start(lines: &[&str]) -> Point {
    let max_x = lines[0].len();
    let max_y = lines.len() - 1;
    for y in 0..max_y {
        for x in 0..max_x {
            if lines[y].chars().nth(x).unwrap() == 'S' {
                return Point { x, y };
            }
        }
    }
    panic!("Invalid entry, no start");
}

fn go_down(grid: &mut Vec<Vec<char>>, pos: Point) -> usize {
    let max_y = grid.len() - 1;

    for y in pos.y + 1..=max_y {
        match grid[y][pos.x] {
            '.' => {
                grid[y][pos.x] = '|';
            }
            '|' => {
                return 1;
            }
            '^' => {
                let mut splitted = 0;
                if pos.x + 1 < grid[y].len() {
                    splitted += go_down(grid, Point { x: pos.x + 1, y });
                }
                if pos.x > 0 {
                    splitted += go_down(grid, Point { x: pos.x - 1, y });
                }
                return splitted;
            }
            _ => panic!("Invalid char"),
        }
    }
    1
}

fn step1(lines: &[&str]) -> usize {
    let mut grid: Vec<Vec<char>> = lines.into_iter().map(|f| f.chars().collect()).collect();

    let start = find_start(lines);

    let r = go_down(&mut grid, start);
    r - 1
}

fn count_path(grid: &Vec<Vec<char>>, pos: Point, visited: &mut HashMap<Point, usize>) -> usize {
    if let Some(&v) = visited.get(&pos) {
        return v;
    }

    let max_y = grid.len() - 1;

    for y in pos.y + 1..=max_y {
        match grid[y][pos.x] {
            '.' => (),
            '|' => {
                visited.insert(pos, 1);
                return 1;
            }
            '^' => {
                let mut path = 0;
                if pos.x + 1 < grid[y].len() {
                    path += count_path(grid, Point { x: pos.x + 1, y }, visited);
                }
                if pos.x > 0 {
                    path += count_path(grid, Point { x: pos.x - 1, y }, visited);
                }
                visited.insert(pos, path);
                return path;
            }
            _ => panic!("Invalid char"),
        }
    }
    visited.insert(pos, 1);
    1
}

fn step2(lines: &[&str]) -> usize {
    let grid: Vec<Vec<char>> = lines.into_iter().map(|f| f.chars().collect()).collect();
    let start = find_start(lines);
    let mut visited = HashMap::new();

    count_path(&grid, start, &mut visited)
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
