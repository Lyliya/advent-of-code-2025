use std::io;
use std::io::Read;
use timer;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

fn parse_coord(lines: &[&str]) -> Vec<Point> {
    let mut coord: Vec<Point> = vec![];

    for line in lines {
        let splitted: Vec<&str> = line.split(',').collect();
        coord.push(Point { x: splitted[0].parse::<i64>().unwrap(), y: splitted[1].parse::<i64>().unwrap() });
    }

    coord
}

fn step1(lines: &[&str]) -> i64 {
    let coords = parse_coord(lines);
    let mut areas: Vec<(i64, Point, Point)> = vec![];

    for (index, coord) in coords.iter().enumerate() {
        for i in index + 1..coords.len() {
            let dx = (coords[i].x - coord.x).abs();
            let dy = (coords[i].y - coord.y).abs();
            let area = (dx + 1) * (dy + 1);
            areas.push((area, *coord, coords[i]));
        }
    }

    areas.sort_by(|(a,_,_), (b,_,_)| b.cmp(a));
    areas[0].0
}

fn on_segment(p: Point, q: Point, r: Point) -> bool {
    q.x >= p.x.min(r.x) && q.x <= p.x.max(r.x) &&
    q.y >= p.y.min(r.y) && q.y <= p.y.max(r.y)
}

fn vec_dir(p: Point, q: Point, r: Point) -> i32 {
    let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
    if val == 0 {
        0 // collinear
    } else if val > 0 {
        1 // clockwise
    } else {
        2 // counterclockwise
    }
}

fn check_point_in_polygon(p: Point, edges: &Vec<Point>) -> bool {
    let mut count = 0;
    let n = edges.len();

    let extreme = Point{x: 100_000, y: p.y};

    for i in 0..n {
        let c = edges[i];
        let d = edges[(i + 1) % n];

        if check_intersect(p, extreme, c, d) {
            if vec_dir(c, d, p) == 0 {
                return on_segment(c, p, d);
            }
            count += 1;
        }
    }

    count % 2 == 1
}

fn check_intersect(a: Point, b: Point, c: Point, d: Point) -> bool {
    let o1 = vec_dir(a, b, c);
    let o2 = vec_dir(a, b, d);
    let o3 = vec_dir(c, d, a);
    let o4 = vec_dir(c, d, b);

    if o1 != o2 && o3 != o4 {
        return true;
    }

    if o1 == 0 && on_segment(a, c, b) { return true; }
    if o2 == 0 && on_segment(a, d, b) { return true; }
    if o3 == 0 && on_segment(c, a, d) { return true; }
    if o4 == 0 && on_segment(c, b, d) { return true; }

    false
}

fn check_corners(a: Point, b: Point, edges: &Vec<Point>) -> bool {
    let xmin = a.x.min(b.x);
    let xmax = a.x.max(b.x);
    let ymin = a.y.min(b.y);
    let ymax = a.y.max(b.y);

    let corners = [
        Point {x: xmin, y: ymin},
        Point {x: xmin, y: ymax},
        Point {x: xmax, y: ymin},
        Point {x: xmax, y: ymax},
    ];

    corners.iter().all(|corner| check_point_in_polygon(*corner, edges))
}

fn check_rectangle(a: Point, b: Point, edges: &Vec<Point>) -> bool {
    for i in 0..edges.len() {
        let c = edges[i];
        let d = edges[(i + 1) % edges.len()];

        // One of the point is egal to the other. This is a triangle
        if c == a || c == b || d == a || d == b {
            continue;
        }

        if check_intersect(a, b, c, d) {
            return false;
        }
    }

    let mid = Point {
        x: (a.x + b.x) / 2,
        y: (a.y + b.y) / 2,
    };
    if !check_point_in_polygon(mid, edges) {
        return false;
    }

    if !check_corners(a, b, edges) {
        return false;
    }

    true
}

fn step2(lines: &[&str]) -> i64 {
    let coords = parse_coord(lines);
    let mut areas: Vec<(i64, Point, Point)> = vec![];

    for (index, coord) in coords.iter().enumerate() {
        for i in index + 1..coords.len() {
            let dx = (coords[i].x - coord.x).abs();
            let dy = (coords[i].y - coord.y).abs();
            let area = (dx + 1) * (dy + 1);
            areas.push((area, *coord, coords[i]));
        }
    }

    areas.sort_by(|(a,_,_), (b,_,_)| b.cmp(a));

    for i in 0..areas.len() {
        if check_rectangle(areas[i].1, areas[i].2, &coords) {
            return areas[i].0
        }
    }
    panic!("Invalid rectangles")
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