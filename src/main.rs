use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum Shape {
    Canvas,
    Circle,
    HorizontalLine,
    VerticalLine,
    DiagonalLineLeftToRight,
    DiagonalLineRightToLeft,
}

fn canvas_index_to_coords(i: u32, num: u32) -> (u32, u32) {
    if i < num { (i, 0) }
    else { (i % num, i / num) }
}

fn write(coords: (u32, u32), chr: char,  num: u32) {
    if coords.0 == num - 1 { println!("{}", chr); }
    else { print!("{} ", chr); }
    // if coords.0 == num - 1 { println!("{} ({}/{})", chr, coords.0, coords.1); }
    // else { print!("{} ({}/{}) ", chr, coords.0, coords.1); }
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
    let x0 = point.0;
    let y0 = point.1;

    let mut x = radius;
    let mut y = 0;

    let mut err: i32 = 0;

    let mut vec = HashMap::new();

    while x >= y {
        vec.insert((x0 + x, y0 + y), Shape::Circle);
        vec.insert((x0 + y, y0 + x), Shape::Circle);
        vec.insert((x0 - y, y0 + x), Shape::Circle);
        vec.insert((x0 - x, y0 + y), Shape::Circle);
        vec.insert((x0 - x, y0 - y), Shape::Circle);
        vec.insert((x0 - y, y0 - x), Shape::Circle);
        vec.insert((x0 + y, y0 - x), Shape::Circle);
        vec.insert((x0 + x, y0 - y), Shape::Circle);

        y += 1;
        err += 1 + 2 * y as i32;

        if 2 * (err - x as i32) + 1 > 0
        {
            x -= 1;
            err += 1 - 2 * x as i32;
        }
    }

    vec
}

fn line_shape(rectangle: (u32, u32, u32, u32)) -> Shape
{
    if rectangle.0 != rectangle.2 && rectangle.1 > rectangle.3 { Shape::DiagonalLineLeftToRight }
    else if rectangle.0 != rectangle.2 && rectangle.1 < rectangle.3 { Shape::DiagonalLineRightToLeft }
    else if rectangle.1 == rectangle.3 { Shape::HorizontalLine }
    else { Shape::VerticalLine }
}

fn line(rectangle: (u32, u32, u32, u32)) -> HashMap<(u32, u32), Shape> {
    // @FIXME this typecasting is pretty ugly,
    // maybe unpack tuple before into signed variables?
    let dx = ((rectangle.2 - rectangle.0) as i32).abs();

    let sx: i32 = if rectangle.0 < rectangle.2 { 1 } else { -1 };

    let dy = ((rectangle.3 - rectangle.1) as i32).abs();
    let sy: i32 = if rectangle.1 < rectangle.3 { 1 } else { -1 };

    let tmp = if dx > dy { dx } else { -dy };
    let mut err = tmp / 2;
    let mut e2;

    let mut x0_m = rectangle.0 as i32;
    let mut y0_m = rectangle.1 as i32;

    let mut coords = HashMap::new();
    let line_shape = line_shape(rectangle);

    loop {
        coords.insert((x0_m as u32, y0_m as u32), line_shape);

        if x0_m == rectangle.2 as i32 && y0_m == rectangle.3 as i32 {
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

fn draw(num: u32, coords: HashMap<(u32, u32), Shape>) {
    let mut vec = Vec::new();

    for (key, value) in &coords {
        vec.push((key.0, key.1, value));
    }

    vec.sort_by_key(|&(x, _, _)| x);
    vec.sort_by_key(|&(_, y, _)| (y as i32) * -1);

    for data in vec {
        match data.2 {
            &Shape::Canvas => write((data.0, data.1), ' ', num),
            &Shape::Circle => write((data.0, data.1), 'o', num),
            &Shape::HorizontalLine => write((data.0, data.1), '-', num),
            &Shape::VerticalLine => write((data.0, data.1), '|', num),
            &Shape::DiagonalLineLeftToRight => write((data.0, data.1), '\\', num),
            &Shape::DiagonalLineRightToLeft => write((data.0, data.1), '/', num),
       }
    }
}

fn main() {
    let num = 10;
    let canvas_size = (num, num);
    let point_1 = (2, 2);
    let point_2 = (3, 4);
    let point_3 = (7, 7);

    draw(num, combine(canvas(canvas_size), combine(circle(1, point_3), combine(circle(1, point_2), combine(circle(1, point_1), line((0, 0, 0, 9)))))));
}
