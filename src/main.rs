use std::collections::HashMap;

#[derive(PartialEq, PartialOrd, Debug)]
struct Point(u32, u32);

#[derive(PartialEq, PartialOrd, Debug)]
struct Dimension(u32, u32);

#[derive(PartialEq, PartialOrd, Debug, Hash, Eq, Clone)]
struct Coordinate(u32, u32);

impl Coordinate {
    // @FIXME make this private and move to canvas???
    fn from_canvas_index(i: u32, canvas_size: &Dimension) -> Coordinate {
        if i < canvas_size.0 { Coordinate(i, 0) }
        else { Coordinate(i % canvas_size.0, i / canvas_size.0) }
    }
}

// only type alias for now, otherwise I need to re-implement all the functions of the HashMap that I use
// should be wrapper struct though
type ShapeCoordinates = HashMap<Coordinate, Shape>; // @FIXME this can soon be a HashSet, because the types know their Shape

trait Plottable {
    fn shape(&self) -> Shape;

    fn coords(&self) -> &ShapeCoordinates;
}

#[derive(Debug)]
struct Canvas(Dimension, ShapeCoordinates);

impl Plottable for Canvas {
    fn shape(&self) -> Shape {
        Shape::Canvas
    }

    fn coords(&self) -> &ShapeCoordinates {
        &self.1
    }
}

#[derive(Debug)]
struct Circle(ShapeCoordinates);

impl Plottable for Circle {
    fn shape(&self) -> Shape {
        Shape::Circle
    }

    fn coords(&self) -> &ShapeCoordinates {
        &self.0
    }
}

#[derive(Debug)]
struct Line(ShapeCoordinates); // @FIXME only one line type for now during this refactoring

impl Plottable for Line {
    fn shape(&self) -> Shape {
        Shape::HorizontalLine // @TODO return line type here
    }

    fn coords(&self) -> &ShapeCoordinates {
        &self.0
    }
}


#[derive(Clone, Copy, Debug)]
enum Shape {
    Canvas,
    Circle,
    HorizontalLine,
    VerticalLine,
    DiagonalLineLeftToRight,
    DiagonalLineRightToLeft,
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

// -- functions --
fn canvas_dimensions(a: &ShapeCoordinates, b: &ShapeCoordinates) -> Dimension {
    let combined = a.into_iter()
        .chain(b.into_iter())
        .map(|(c, _)| c)
        .collect::<Vec<(&Coordinate)>>();

    let coord_max_x = combined.iter().max_by_key(|&coords| coords.0).unwrap();
    let coord_max_y = combined.iter().max_by_key(|&coords| coords.1).unwrap();

    Dimension(coord_max_x.0 + 1, coord_max_y.1 + 1)
}

fn plot_on_canvas<T: Plottable, U: Plottable>(a: &T, b: &U) -> Canvas {
    let canvas_dimension = canvas_dimensions(a.coords(), b.coords());
    let canvas = new_canvas(&canvas_dimension);

    // @FIXME: this is quite ugly, bu the new hashmap needs to have ownership of the values from the old one
    let coords = canvas.iter().map(|a| (a.0.clone(), a.1.clone()))
        .chain(a.coords().iter().map(|a| (a.0.clone(), a.1.clone())))
        .chain(b.coords().iter().map(|a| (a.0.clone(), a.1.clone())))
        .collect();

    Canvas(canvas_dimension, coords)
}

fn new_canvas(size: &Dimension) -> ShapeCoordinates {
    (0..(size.0 * size.1))
        .map(|i| { (Coordinate::from_canvas_index(i, &size), Shape::Canvas) })
        .collect()
}

fn circle(radius: u32, point: &Point) -> Circle {
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

    Circle(coords)
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

fn line(start: &Point, end: &Point) -> Line {
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

    Line(coords)
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

fn main() {
    let canvas_dimension = Dimension(10, 10);

    let point_1 = Point(2, 2);
    let point_2 = Point(3, 4);
    let point_3 = Point(7, 7);

    let line_start = Point(0, 0);
    let line_end = Point(0, 9);

    let mut canvas = plot_on_canvas(&circle(1, &point_1), &line(&line_start, &line_end));
    canvas = plot_on_canvas(&canvas, &circle(1, &point_2));
    canvas = plot_on_canvas(&canvas, &circle(1, &point_3));

    draw(&canvas , &write);
}
