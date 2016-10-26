use std::collections::HashMap;
use std::cmp::Ordering;

enum Shape {
    Canvas,
    Circle,
}

fn canvas_index_to_coords(i: u32, num: u32) -> (u32, u32) {
    match i {
        i if i < num => { (i, 0) },
        _ => { (i % num, i / num) },
    }
}

fn write(coords: (u32, u32), chr: char,  num: u32) {
    if coords.1 == num - 1 { println!("{}", chr); }
    else { print!("{} ", chr); }
}

fn combine(a: HashMap<(u32, u32), Shape>, b: HashMap<(u32, u32), Shape>) -> HashMap<(u32, u32), Shape> {
    let mut combined = HashMap::new();

    for (key, val) in a {
        combined.insert(key, val);
    }

    for (key, val) in b {
        combined.insert(key, val);
    }

    combined
}

fn canvas(size: (u32, u32)) -> HashMap<(u32, u32), Shape> {
    let mut canvas_coords = HashMap::new();

    for i in 0..(size.0 * size.1) {
        canvas_coords.insert(canvas_index_to_coords(i, size.0), Shape::Canvas);
    }

    canvas_coords
}

fn circle(radius: u32, point: (u32, u32)) -> HashMap<(u32, u32), Shape> {
    let x0 = point.0 as i32;
    let y0 = point.1 as i32;

    let mut x = radius as i32;
    let mut y = 0;

    let mut err = 0;

    let mut vec = HashMap::new();

    // @FIXME this casting back and forth is really nasty, as it's actually only needed once
    while x >= y {
        vec.insert(((x0 + x) as u32, (y0 + y) as u32), Shape::Circle);
        vec.insert(((x0 + y) as u32, (y0 + x) as u32), Shape::Circle);
        vec.insert(((x0 - y) as u32, (y0 + x) as u32), Shape::Circle);
        vec.insert(((x0 - x) as u32, (y0 + y) as u32), Shape::Circle);
        vec.insert(((x0 - x) as u32, (y0 - y) as u32), Shape::Circle);
        vec.insert(((x0 - y) as u32, (y0 - x) as u32), Shape::Circle);
        vec.insert(((x0 + y) as u32, (y0 - x) as u32), Shape::Circle);
        vec.insert(((x0 + x) as u32, (y0 - y) as u32), Shape::Circle);

        y += 1;
        err += 1 + 2 * y;

        if 2 * (err - x) + 1 > 0
        {
            x -= 1;
            err += 1 - 2 * x;
        }
    }

    vec
}

fn draw(num: u32, coords: HashMap<(u32, u32), Shape>) {
    let mut vec = Vec::new();

    for (key, value) in &coords {
        vec.push((key.0, key.1, value));
    }

    vec.sort_by(|a, b| {
        if a.0 > b.0 { return Ordering::Less; }
        if a.0 < b.0 { return Ordering::Greater; }
        if a.1 > b.1 { return Ordering::Greater; }
        if a.1 < b.1 { return Ordering::Less; }
        return Ordering::Equal;
    });

    for data in vec {
        match data.2 {
            &Shape::Canvas => write((data.0, data.1), ' ', num),
            &Shape::Circle => write((data.0, data.1), 'o', num),
        }
    }
}

fn main() {
    let num = 10;
    let canvas_size = (num, num);
    let point_1 = (2, 2);
    let point_2 = (3, 4);
    let point_3 = (7, 7);

    draw(num, combine(canvas(canvas_size), combine(circle(1, point_3), combine(circle(1, point_2), circle(1, point_1)))));
}
