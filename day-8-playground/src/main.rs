use std::io;
use std::io::Read;
use timer;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

fn parse_coord(lines: &[&str]) -> Vec<Point> {
    let mut coord: Vec<Point> = vec![];

    for line in lines {
        let splitted: Vec<&str> = line.split(',').collect();
        coord.push(Point { x: splitted[0].parse::<usize>().unwrap(), y: splitted[1].parse::<usize>().unwrap(), z: splitted[2].parse::<usize>().unwrap() });
    }

    coord
}

fn euclidian_distance(a: &Point, b: &Point) -> f64 {
    let dx = a.x as f64 - b.x as f64;
    let dy = a.y as f64 - b.y as f64;
    let dz = a.z as f64 - b.z as f64;

    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn check_connection(circuits: &mut Vec<Vec<Point>>, p1: Point, p2: Point) {
    let mut c1_index = None;
    let mut c2_index = None;

    for (idx, circuit) in circuits.iter().enumerate() {
        if circuit.contains(&p1) {
            c1_index = Some(idx);
        }
        if circuit.contains(&p2) {
            c2_index = Some(idx);
        }
    }

    match (c1_index, c2_index) {
        (None, None) => {
            circuits.push(vec![p1, p2]);
        }

        (Some(i), None) => {
            circuits[i].push(p2);
        }

        (None, Some(i)) => {
            circuits[i].push(p1);
        }

        (Some(i1), Some(i2)) => {
            if i1 == i2 {
                return;
            }

            let (keep_idx, remove_idx) = if i1 < i2 { (i1, i2) } else { (i2, i1) };

            let to_merge = circuits.remove(remove_idx);
            let target = &mut circuits[keep_idx];

            for p in to_merge {
                if !target.contains(&p) {
                    target.push(p);
                }
            }
        }
    }
}

fn step1(lines: &[&str]) -> usize {
    let exit_loop = 1000;
    let coords = parse_coord(lines);
    let mut distances: Vec<(f64, Point, Point)> = vec![];

    for (index, coord) in coords.iter().enumerate() {
        for i in index + 1..coords.len() {
            let dist = euclidian_distance(coord, &coords[i]);
            distances.push((dist, *coord, coords[i]));
        }
    }

    distances.sort_by(|(a, _, _), (b, _, _)| a.partial_cmp(b).unwrap());


    let mut circuits: Vec<Vec<Point>> = vec![];
    for i in 0..exit_loop {
        let p1 = distances[i].1;
        let p2 = distances[i].2;
        check_connection(&mut circuits, p1, p2);
    }

    circuits.sort_by(|a, b| b.len().partial_cmp(&a.len()).unwrap());
    let mut answer = 1;
    for i in 0..3 {
        answer *= circuits[i].len();
    }
    answer
}

fn step2(lines: &[&str]) -> usize {
    let coords = parse_coord(lines);
    let mut distances: Vec<(f64, Point, Point)> = vec![];

    for (index, coord) in coords.iter().enumerate() {
        for i in index + 1..coords.len() {
            let dist = euclidian_distance(coord, &coords[i]);
            distances.push((dist, *coord, coords[i]));
        }
    }

    distances.sort_by(|(a, _, _), (b, _, _)| a.partial_cmp(b).unwrap());


    let mut circuits: Vec<Vec<Point>> = vec![];
    for i in 0..distances.len() {
        let p1 = distances[i].1;
        let p2 = distances[i].2;

        check_connection(&mut circuits, p1, p2);

        if circuits.len() == 1 && circuits[0].len() == coords.len() {
            return p1.x * p2.x;
        }
    }
    panic!("Invalid entry, no loop detected")
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
