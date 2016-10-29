use std::collections::HashMap;

#[derive(PartialEq, PartialOrd, Debug)]
struct Point(u32, u32);

#[derive(PartialEq, PartialOrd, Debug)]
struct Dimension(u32, u32);

#[derive(PartialEq, PartialOrd, Debug, Hash, Eq)]
struct Coordinate(u32, u32);

#[derive(Clone, Copy, Debug)]
enum Shape {
    Canvas,
    Circle,
    HorizontalLine,
    VerticalLine,
    DiagonalLineLeftToRight,
    DiagonalLineRightToLeft,
}

fn canvas_index_to_coords(i: u32, num: u32) -> Coordinate {
    if i < num { Coordinate(i, 0) }
    else { Coordinate(i % num, i / num) }
}

fn write(coords: &Coordinate, chr: char,  num: u32) {
    if coords.0 == num - 1 { println!("{}", chr); }
    else { print!("{} ", chr); }
    // if coords.0 == num - 1 { println!("{} ({}/{})", chr, coords.0, coords.1); }
    // else { print!("{} ({}/{}) ", chr, coords.0, coords.1); }
}

fn combine(a: HashMap<Coordinate, Shape>, b: HashMap<Coordinate, Shape>) -> HashMap<Coordinate, Shape> {
    let mut combined = HashMap::new();

    for (key, val) in a {
        combined.insert(key, val);
    }

    for (key, val) in b {
        combined.insert(key, val);
    }

    combined
}

fn canvas(size: Dimension) -> HashMap<Coordinate, Shape> {
    let mut canvas_coords = HashMap::new();

    for i in 0..(size.0 * size.1) {
        canvas_coords.insert(canvas_index_to_coords(i, size.0), Shape::Canvas);
    }

    canvas_coords
}

fn circle(radius: u32, point: Point) -> HashMap<Coordinate, Shape> {
    let x0 = point.0;
    let y0 = point.1;

    let mut x = radius;
    let mut y = 0;

    let mut err: i32 = 0;

    let mut coords = HashMap::new();

    while x >= y {
        coords.insert(Coordinate(x0 + x, y0 + y), Shape::Circle);
        coords.insert(Coordinate(x0 + y, y0 + x), Shape::Circle);
        coords.insert(Coordinate(x0 - y, y0 + x), Shape::Circle);
        coords.insert(Coordinate(x0 - x, y0 + y), Shape::Circle);
        coords.insert(Coordinate(x0 - x, y0 - y), Shape::Circle);
        coords.insert(Coordinate(x0 - y, y0 - x), Shape::Circle);
        coords.insert(Coordinate(x0 + y, y0 - x), Shape::Circle);
        coords.insert(Coordinate(x0 + x, y0 - y), Shape::Circle);

        y += 1;
        err += 1 + 2 * y as i32;

        if 2 * (err - x as i32) + 1 > 0
        {
            x -= 1;
            err += 1 - 2 * x as i32;
        }
    }

    coords
}

fn line_shape(start: Point, end: Point) -> Shape {
    let x0 = start.0 as i32;
    let y0 = start.1 as i32;
    let x1 = end.0 as i32;
    let y1 = end.1 as i32;

    if x0 != x1 && y0 > y1 { Shape::DiagonalLineLeftToRight }
    else if x0 != x1 && y0 < y1 { Shape::DiagonalLineRightToLeft }
    else if y0 == y1 { Shape::HorizontalLine }
    else { Shape::VerticalLine }
}

fn line(start: Point, end: Point) -> HashMap<Coordinate, Shape> {
    let x0 = start.0 as i32;
    let y0 = start.1 as i32;
    let x1 = end.0 as i32;
    let y1 = end.1 as i32;

    let dx = ((x1 - x0)).abs();

    let sx: i32 = if x0 < x1 { 1 } else { -1 };

    let dy = ((y1 - y0)).abs();
    let sy: i32 = if y0 < y1 { 1 } else { -1 };

    let tmp = if dx > dy { dx } else { -dy };
    let mut err = tmp / 2;
    let mut e2;

    let mut x0_m = x0;
    let mut y0_m = y0;

    let mut coords = HashMap::new();
    let line_shape = line_shape(start, end);

    loop {
        coords.insert(Coordinate(x0_m as u32, y0_m as u32), line_shape);

        if x0_m == x1 as i32 && y0_m == y1 as i32 {
            break;
        }

        e2 = err;

        if e2 > -dx {
            err -= dy;
            x0_m += sx;
        }

        if e2 < dy {
            err += dx;
            y0_m += sy;
        }
    }

    coords
}

fn draw(num: u32, coords: HashMap<Coordinate, Shape>) {
    let mut vec = Vec::new();

    for (key, value) in &coords {
        vec.push((key, value));
    }

    vec.sort_by_key(|&(coord, _)| coord.0);
    vec.sort_by_key(|&(coord, _)| (coord.1 as i32) * -1);

    for (coord, shape) in vec {
        match shape {
            &Shape::Canvas => write(coord, ' ', num),
            &Shape::Circle => write(coord, 'o', num),
            &Shape::HorizontalLine => write(coord, '-', num),
            &Shape::VerticalLine => write(coord, '|', num),
            &Shape::DiagonalLineLeftToRight => write(coord, '\\', num),
            &Shape::DiagonalLineRightToLeft => write(coord, '/', num),
        }
    }
}

fn main() {
    let num = 10;
    let canvas_size = Dimension(num, num);
    let point_1 = Point(2, 2);
    let point_2 = Point(3, 4);
    let point_3 = Point(7, 7);
    let line_start = Point(0, 0);
    let line_end = Point(0, 9);

    draw(num, combine(canvas(canvas_size), combine(circle(1, point_3), combine(circle(1, point_2), combine(circle(1, point_1), line(line_start, line_end))))));
}
