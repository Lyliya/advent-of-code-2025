use std::io;
use std::io::Read;
use timer;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn index_to_point(index: usize, len: usize) -> Point {
    Point {
        x: index % len,
        y: index / len,
    }
}

fn check_around(grid: &Vec<Vec<char>>, pos: &Point) -> usize {
    let mut occurences: usize = 0;
    let right_boundary = grid[0].len() - 1;
    let bottom_boundary = grid.len() - 1;

    // CHECK TOP LEFT
    if pos.x > 0 && pos.y > 0 && grid[pos.y - 1][pos.x - 1] == '@' {
        occurences += 1;
    }
    // CHECK TOP
    if pos.y > 0 && grid[pos.y - 1][pos.x] == '@' {
        occurences += 1;
    }
    // CHECK TOP RIGHT
    if pos.x < right_boundary && pos.y > 0 && grid[pos.y - 1][pos.x + 1] == '@' {
        occurences += 1;
    }
    // CHECK LEFT
    if pos.x > 0 && grid[pos.y][pos.x - 1] == '@' {
        occurences += 1;
    }
    // CHECK RIGHT
    if pos.x < right_boundary && grid[pos.y][pos.x + 1] == '@' {
        occurences += 1;
    }
    // CHECK BOTTOM
    if pos.y < bottom_boundary && grid[pos.y + 1][pos.x] == '@' {
        occurences += 1;
    }
    // CHECK BOTTOM LEFT
    if pos.x > 0 && pos.y < bottom_boundary && grid[pos.y + 1][pos.x - 1] == '@' {
        occurences += 1;
    }
    // CHECK BOTTOM RIGHT
    if pos.x < right_boundary && pos.y < bottom_boundary && grid[pos.y + 1][pos.x + 1] == '@' {
        occurences += 1;
    }
    occurences
}

fn step1(lines: &[&str]) -> usize {
    let mut answer = 0;
    let grid: Vec<Vec<char>> = lines.into_iter().map(|f| f.chars().collect()).collect();

    for (i, c) in lines.join("").chars().enumerate() {
        if c == '@' {
            let pos = index_to_point(i, grid[0].len());
            let check = check_around(&grid, &pos);
            if check < 4 {
                answer += 1;
            }
        }
    }
    answer
}

fn check_grid(grid: &Vec<Vec<char>>, line: &str) -> Vec<Point> {
    let mut roll: Vec<Point> = vec![];

    for (i, c) in line.chars().enumerate() {
        if c == '@' {
            let pos = index_to_point(i, grid[0].len());
            let check = check_around(&grid, &pos);
            if check < 4 {
                roll.push(pos);
            }
        }
    }
    roll
}

fn step2(lines: &[&str]) -> usize {
    let mut answer = 0;
    let mut grid: Vec<Vec<char>> = lines.into_iter().map(|f| f.chars().collect()).collect();
    let line: String = grid.iter().flat_map(|row| row.iter()).collect();

    let mut roll = check_grid(&grid, &line);
    while roll.len() > 0 {
        answer += roll.len();
        for r in roll.iter() {
            grid[r.y][r.x] = '.';
        }
        let line: String = grid.iter().flat_map(|row| row.iter()).collect();
        roll = check_grid(&grid, &line);

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

