use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
enum ObjectType {
    Canvas,
    Circle,
    Line,
}

impl ObjectType {
    fn to_char(&self) -> char {
        match *self {
            ObjectType::Canvas => ' ',
            ObjectType::Circle => 'o',
            ObjectType::Line => '-',
        }
    }
}

// http://codereview.stackexchange.com/questions/110161/binary-trees-in-rust-iterators
trait Plottable {
    fn object_type(&self) -> ObjectType;

    fn children(&self) -> &Vec<Box<Plottable>>;

    fn coords(&self) -> HashSet<(u32, u32)>;
}

struct Shape(ObjectType, HashSet<(u32, u32)>, Vec<Box<Plottable>>);

impl Shape {
    fn new_circle(radius: u32, point: (u32, u32)) -> Shape {
        let (x0, y0) = point;

        let mut x = radius;
        let mut y = 0;

        let mut err: i32 = 0;

        let mut coords = HashSet::new();

        while x >= y {
            coords.insert((x0 + x, y0 + y));
            coords.insert((x0 + y, y0 + x));
            coords.insert((x0 - y, y0 + x));
            coords.insert((x0 - x, y0 + y));
            coords.insert((x0 - x, y0 - y));
            coords.insert((x0 - y, y0 - x));
            coords.insert((x0 + y, y0 - x));
            coords.insert((x0 + x, y0 - y));

            y += 1;
            err += 1 + 2 * y as i32;

            if 2 * (err - x as i32) + 1 > 0
            {
                x -= 1;
                err += 1 - 2 * x as i32;
            }
        }

        Shape(ObjectType::Circle, coords, Vec::new())
    }

    fn new_line(start: (u32, u32), end: (u32, u32)) -> Shape {
        fn line_shape(start: (u32, u32), end: (u32, u32)) -> ObjectType {
            ObjectType::Line
            // let &Point(x0, y0) = start;
            // let &Point(x1, y1) = end;

            // if y0 == y1 {
            //     Shape::HorizontalLine
            // } else if x0 == x1 {
            //     Shape::VerticalLine
            // } else if y0 > y1 {
            //     Shape::DiagonalLineLeftToRight
            // } else {
            //     Shape::DiagonalLineRightToLeft
            // }
        }

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

        let mut coords = HashSet::new();
        let line_shape = line_shape(start, end);

        loop {
            coords.insert((x0_m as u32, y0_m as u32));

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

        Shape(ObjectType::Line, coords, Vec::new())
    }

    fn new_empty_canvas(dimension: (u32, u32)) -> Shape {
        Shape(ObjectType::Canvas, Shape::calc_coords(dimension), Vec::new())
    }

    fn combine(a: Box<Plottable>, b: Box<Plottable>) -> Shape {
        let mut d = Vec::new();

        d.push(a);
        d.push(b);

        // let tmp_canvas = Shape(ObjectType::Canvas, HashSet::new(), d);

        // let b: Box<Plottable> = Box::new(tmp_canvas);
        let canvas_coords = Shape::wrap_in_canvas(&d);

        Shape(ObjectType::Canvas, canvas_coords, d)
    }

    // @TOD make private
    fn collect_coords(a: &Box<Plottable>, vec: &mut Vec<(u32, u32, ObjectType)>, depth: u32) {
        //println!("{} ObjectType: {:?}", (0..depth).map(|_| '\t').collect::<String>(), a.object_type());

        for z in a.coords() {
            vec.push((z.0, z.1, a.object_type()));
        }

        let new_depth = depth + 1;

        for c in a.children() {
            Shape::collect_coords(c.clone(), vec, new_depth);
        }

        //println!("{} Adding my coords at depth {}", (0..depth).map(|_| '\t').collect::<String>(), depth);
    }

    // @TODO make private
    fn draw_canvas(a: Box<Plottable>) {
        let mut vec = Vec::new();
        Shape::collect_coords(&a, &mut vec, 0);

        println!("{:?}", vec);

        let mut unique_coords = HashMap::new();

        for (x, y, shape) in vec {
            unique_coords.insert((x, y), shape);
        }

        let mut x = unique_coords.into_iter().collect::<Vec<_>>();
        x.sort_by_key(|&(coords, _)| (!coords.1, coords.0));

        println!("{:?}", x);

        let (max_x, _) = Shape::calculate_max_dimensions_iter(x.iter());

        for ((x, y), shape) in x {
            let chr = shape.to_char();
            write((x, y), chr, max_x);
        }
    }

    fn calculate_max_dimensions_iter(combined: std::slice::Iter<((u32, u32), ObjectType)>) -> (u32, u32) {
        // this is still quite ugly actually
        let coord_max_x = combined.clone().max_by_key(|&coords| (coords.0).0).unwrap();
        let coord_max_y = combined.clone().max_by_key(|&coords| (coords.0).1).unwrap();

        ((coord_max_x.0).0 + 1, (coord_max_y.0).1 + 1)
    }

    fn calculate_max_dimensions(a: HashMap<(u32, u32), ObjectType>) -> (u32, u32) {
        let combined = a.keys().into_iter().collect::<Vec<&(u32, u32)>>();

        // this is still quite ugly actually
        let coord_max_x = combined.iter().max_by_key(|&coords| coords.0).unwrap();
        let coord_max_y = combined.iter().max_by_key(|&coords| coords.1).unwrap();

        (coord_max_x.0 + 1, coord_max_y.1 + 1)
    }

    fn from_canvas_index(i: u32, canvas_size: (u32, u32)) -> (u32, u32) {
        if i < canvas_size.0 { (i, 0) }
        else { (i % canvas_size.0, i / canvas_size.0) }
    }

    // @TODO make this private
    fn calc_coords(dimension: (u32, u32)) -> HashSet<(u32, u32)> {
        (0..(dimension.0 * dimension.1))
            .map(|i| { Shape::from_canvas_index(i, dimension) })
            .collect()
    }

    fn wrap_in_canvas(a: &Vec<Box<Plottable>>) -> HashSet<(u32, u32)> {
        let mut vec = Vec::new();

        for z in a {
            Shape::collect_coords(z, &mut vec, 0);
        }

        let mut unique_coords = HashMap::new();

        for (x, y, shape) in vec {
            unique_coords.insert((x, y), shape);
        }

        let canvas_dimension = Shape::calculate_max_dimensions(unique_coords);
        Shape::calc_coords(canvas_dimension)


        // let canvas = Shape::new_empty_canvas(canvas_dimension);

        // let b: Box<Plottable> = Box::new(canvas);
        // Shape::combine(b, a)
    }

    // @TODO make this private
    // @TODO fix me, this code duplicates and is kind of ugly
    fn draw_wrap_canvas(a: Box<Plottable>) {
        let mut vec = Vec::new();
        Shape::collect_coords(&a, &mut vec, 0);

        let mut unique_coords = HashMap::new();

        for (x, y, shape) in vec {
            unique_coords.insert((x, y), shape);
        }

        let canvas_dimension = Shape::calculate_max_dimensions(unique_coords);
        let canvas = Shape::new_empty_canvas(canvas_dimension);

        let b: Box<Plottable> = Box::new(canvas);
        let canvas_combined = Shape::combine(b, a);

        let c: Box<Plottable> = Box::new(canvas_combined);
        Shape::draw(c);
    }

    fn draw(a: Box<Plottable>) {
        match a.object_type() {
            ObjectType::Canvas => Shape::draw_canvas(a),
            _ => Shape::draw_wrap_canvas(a),
        }
    }
}

impl Plottable for Shape {
    fn object_type(&self) -> ObjectType {
        self.0
    }

    fn coords(&self) -> HashSet<(u32, u32)> {
        self.1.clone() // @TODO should this return a reference?
    }

    fn children(&self) -> &Vec<Box<Plottable>> {
        &self.2
    }
}

fn write(coords: (u32, u32), chr: char, line_length: u32) {
    if coords.0 == line_length - 1 { println!("{}", chr); }
    else { print!("{} ", chr); }
}

fn write_debug(coords: (u32, u32), chr: char, line_length: u32) {
    if coords.0 == line_length - 1 { println!("{} ({}/{})", chr, coords.0, coords.1); }
    else { print!("{} ({}/{}) ", chr, coords.0, coords.1); }
}

fn main() {
    let point_1 = (2, 2);
    let point_2 = (3, 4);
    let point_3 = (7, 7);

    let line_start_1 = (0, 0);
    let line_end_1 = (9, 0);

    let line_start_2 = (0, 9);
    let line_end_2 = (3, 9);

    // let empty_canvas = Shape::new_empty_canvas(canvas_dimension);

    let line_1 = Shape::new_line(line_start_1, line_end_1);
    let line_2 = Shape::new_line(line_start_2, line_end_2);

    let circle_1 = Shape::new_circle(1, point_1);
    let circle_2 = Shape::new_circle(1, point_2);
    let circle_3 = Shape::new_circle(1, point_3);

    let mut canvas = Shape::combine(Box::new(circle_1), Box::new(line_1));
    let mut tmp_canvas = Shape::combine(Box::new(circle_2), Box::new(circle_3));
    canvas = Shape::combine(Box::new(canvas), Box::new(tmp_canvas));

    let b: Box<Plottable> = Box::new(canvas);
    Shape::draw(b);
}
