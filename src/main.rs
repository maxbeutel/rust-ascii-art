use std::collections::HashMap;

#[derive(PartialEq, PartialOrd, Debug)]
struct Point(u32, u32);

#[derive(PartialEq, PartialOrd, Debug)]
struct Dimension(u32, u32);

#[derive(PartialEq, PartialOrd, Debug, Hash, Eq, Clone)]
struct Coordinate(u32, u32);

type ShapeCoordinates = HashMap<Coordinate, Shape>;

#[derive(Debug)]
struct Canvas(Dimension, ShapeCoordinates);

#[derive(Clone, Copy, Debug)]
enum Shape {
    Canvas,
    Circle,
    HorizontalLine,
    VerticalLine,
    DiagonalLineLeftToRight,
    DiagonalLineRightToLeft,
}

impl Coordinate {
    // @FIXME make this private and move to canvas???
    fn from_canvas_index(i: u32, canvas_size: &Dimension) -> Coordinate {
        if i < canvas_size.0 { Coordinate(i, 0) }
        else { Coordinate(i % canvas_size.0, i / canvas_size.0) }
    }
}

impl Shape {
    fn to_char(&self) -> char {
        match *self {
            Shape::Canvas => ' ',
            Shape::Circle => 'o',
            Shape::HorizontalLine => '-',
            Shape::VerticalLine => '|',
            Shape::DiagonalLineLeftToRight => '\\',
            Shape::DiagonalLineRightToLeft => '/',
        }
    }
}

fn canvas_dimensions(a: &ShapeCoordinates, b: &ShapeCoordinates) -> Dimension {
    let combined = a.into_iter()
        .chain(b.into_iter())
        .map(|(c, _)| c)
        .collect::<Vec<(&Coordinate)>>();

    //println!("combined: {:?}", combined);

    let coord_max_x = combined.iter().max_by_key(|&coords| coords.0).unwrap();
    let coord_max_y = combined.iter().max_by_key(|&coords| coords.1).unwrap();

    Dimension(coord_max_x.0 + 1, coord_max_y.1 + 1)
}

fn combine(a: ShapeCoordinates, b: ShapeCoordinates) -> Canvas {
    let canvas_dimension = canvas_dimensions(&a, &b);
    let canvas = canvas(&canvas_dimension);

    // let coords = a.into_iter().chain(b.into_iter()).chain(canvas.into_iter()).collect();

    let coords = canvas.into_iter().chain(a.into_iter()).chain(b.into_iter()).collect();
    //println!("dim {:?}", canvas_dimension);
    Canvas(canvas_dimension, coords)
}

// let a_max_coord_by_x = (a.iter().max_by_key(|&(coords, _)| coords.0).unwrap().0).0;
// let a_max_coord_by_y = (a.iter().max_by_key(|&(coords, _)| coords.1).unwrap().0).1;

// let b_max_coord_by_x = (b.iter().max_by_key(|&(coords, _)| coords.0).unwrap().0).0;
// let b_max_coord_by_y = (b.iter().max_by_key(|&(coords, _)| coords.1).unwrap().0).1;

// println!("a_max = {:?}", (a_max_coord_by_x, a_max_coord_by_y));
// println!("b_max = {:?}", (b_max_coord_by_x, b_max_coord_by_y));


// let canvas_dimension_x = [a_max_coord_by_x, b_max_coord_by_x].into_iter().max();
// let canvas_dimension_y = 0;

//    a.into_iter().chain(b.into_iter()).collect()

fn canvas(size: &Dimension) -> ShapeCoordinates {
    // println!("creating {} coords", size.0 * size.1);

    (0..(size.0 * size.1))
        .map(|i| { (Coordinate::from_canvas_index(i, &size), Shape::Canvas) })
        .collect()
}

fn circle(radius: u32, point: &Point) -> ShapeCoordinates {
    let &Point(x0, y0) = point;

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

fn line_shape(start: &Point, end: &Point) -> Shape {
    let &Point(x0, y0) = start;
    let &Point(x1, y1) = end;

    if y0 == y1 {
        Shape::HorizontalLine
    } else if x0 == x1 {
        Shape::VerticalLine
    } else if y0 > y1 {
        Shape::DiagonalLineLeftToRight
    } else {
        Shape::DiagonalLineRightToLeft
    }
}

fn line(start: &Point, end: &Point) -> ShapeCoordinates {
    // how to make this nicer and use tuple deconstruction?
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

fn draw(canvas: &Canvas, write_fn: &Fn(&Coordinate, char, u32)) {
    // @TODO make this work, borrow error
    // let &Canvas(canvas_dimension, coords) = canvas;

    let mut vec = canvas.1.iter().collect::<Vec<_>>();
    vec.sort_by_key(|&(coords, _)| (!coords.1, coords.0)); // ?

    //println!("break at x = {}", (canvas.0).0);

    for (coords, shape) in vec {
        let chr = shape.to_char();
        write_fn(coords, chr, (canvas.0).0);
    }
}

fn write(coords: &Coordinate, chr: char, line_length: u32) {
    if coords.0 == line_length - 1 { println!("{}", chr); }
    else { print!("{} ", chr); }
}

fn write_debug(coords: &Coordinate, chr: char, line_length: u32) {
    if coords.0 == line_length - 1 { println!("{} ({}/{})", chr, coords.0, coords.1); }
    else { print!("{} ({}/{}) ", chr, coords.0, coords.1); }
}

// fn plot(canvas_dimension: Dimension, coords: ShapeCoordinates) -> Canvas {
//     let all_coords_on_canvas = canvas(&canvas_dimension);
//     let plotted_coords = combine(all_coords_on_canvas, coords);
//     Canvas(canvas_dimension, plotted_coords)
// }

fn main() {
    let num = 10;
    let canvas_dimension = Dimension(3, 9);

    let point_1 = Point(2, 2);
    let point_2 = Point(3, 4);
    let point_3 = Point(7, 7);

    let line_start = Point(0, 0);
    let line_end = Point(0, 9);

    let mut canvas = combine(circle(1, &point_1), line(&line_start, &line_end));
    canvas = combine(circle(1, &point_2), canvas);
    // coords = ;
    // coords = ;

    draw(&canvas , &write);
}
