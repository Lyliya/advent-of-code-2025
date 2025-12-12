use std::collections::HashMap;
use std::io;
use std::io::Read;
use timer;

fn parse_shape(lines: &[&str]) -> HashMap<usize, usize> {
    let mut shapes = HashMap::new();

    for shape in lines {
        let splitted: Vec<&str> = shape.split("\n").collect();
        let id = splitted[0].replace(":", "").parse::<usize>().unwrap();
        let mut area = 0;
        for r in &splitted[1..splitted.len()] {
            area += r.matches('#').count();
        }
        shapes.insert(id, area);
    }

    shapes
}

fn parse_grid(grids: &[&str]) -> Vec<(usize, Vec<usize>)> {
    let mut res = vec![];
    for grid in grids {
        let (left, right) = grid.split_once(": ").unwrap();
        let total_area: usize = left
            .split_once('x')
            .map(|(a, b)| a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap())
            .unwrap();

        let shapes: Vec<usize> = right.trim().split_whitespace().map(|v| v.parse::<usize>().unwrap()).collect();
        res.push((total_area, shapes));
    }

    res
}

fn step1(lines: &String) -> usize {
    let mut res = 0;
    let splitted: Vec<&str> = lines.split("\n\n").collect();
    let shapes = parse_shape(&splitted[0..splitted.len() - 1]);

    let grids: Vec<&str> = splitted.last().unwrap().trim().split("\n").collect();
    let grids = parse_grid(&grids);

    for (total_area, grid_shapes) in grids {
        let mut shape_area = 0;
        for (i, shape) in grid_shapes.iter().enumerate() {
            shape_area += shapes.get(&i).unwrap() * shape;
        }
        if shape_area <= total_area {
            res += 1;
        }
    }

    res
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let (step1_answer, step1_time) = timer::measure(|| step1(&input));
    println!("Step 1 answer: {}, in {:?}", step1_answer, step1_time);
}