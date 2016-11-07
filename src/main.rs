//use std::collections::HashSet;
// @TODO wanna use HashSet as coords can only be unique, see
// http://stackoverflow.com/questions/27828487/hashmap-with-hashset-as-key
// (need to implement Hash for Hashmap ...)


#[derive(Debug, PartialEq, Clone)]
enum Representation {
    Canvas,
    Line,
    Circle,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Coords(u32, u32);

#[derive(Debug, PartialEq, Clone)]
struct PlottedCoords(u32, u32, Representation);

#[derive(Debug, PartialEq, Copy, Clone)]
struct Dimensions(u32, u32);


// -- Plottable objects --
trait Plottable {
    // @TODO it's not really clear that get_dimensions() returns the canvas size needed for the object, not the *actual* dimensions for the object
// a horizontal line at (1, 1) to (3, 1) has the canvas size (4, 2) and dimension is actually (3, 1). get_dimensions at the moment will return (4, 2)
    fn get_dimensions(&self) -> Dimensions;

    fn get_coords(&self) -> Vec<Coords>;

    fn get_representation_at(&self, coords: Coords) -> Option<Representation>;
}

struct CombinedObject(Dimensions, Vec<Box<Plottable>>);

impl Plottable for CombinedObject {
    fn get_dimensions(&self) -> Dimensions {
        self.0
    }

    fn get_coords(&self) -> Vec<Coords> {
        let mut coords = vec![];

        for contained_plottable in self.1.iter() {
            let contained_coords = contained_plottable.get_coords();
            coords.extend(contained_coords.clone());
        }

        coords
    }

    fn get_representation_at(&self, coords: Coords) -> Option<Representation> {
        for contained_plottable in self.1.iter() {
            let contained_coords = contained_plottable.get_coords();
            let result = contained_coords.iter().find(|a| **a == coords);

            match result {
                Some(matched_contained_coords) => { return contained_plottable.get_representation_at(*matched_contained_coords) },
                None => {},
            };
        }

        None
    }
}


#[derive(Debug)]
struct Line(Dimensions, Vec<Coords>);

impl Line {
    fn new(start: Coords, end: Coords) -> Line {
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

        let mut coords = vec![];

        loop {
            coords.push(Coords(x0_m as u32, y0_m as u32));

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

        let dimensions = get_dimensions_from_coords(&coords);
        Line(dimensions, coords)
    }
}

impl Plottable for Line {
    fn get_dimensions(&self) -> Dimensions {
        self.0
    }

    fn get_coords(&self) -> Vec<Coords> {
        self.1.clone()
    }

    fn get_representation_at(&self, _: Coords) -> Option<Representation> {
        Some(Representation::Line)
    }
}

#[derive(Debug)]
struct Circle(Dimensions, Vec<Coords>);

impl Plottable for Circle {
    fn get_dimensions(&self) -> Dimensions {
        self.0
    }

    fn get_coords(&self) -> Vec<Coords> {
        self.1.clone()
    }

    fn get_representation_at(&self, _: Coords) -> Option<Representation> {
        Some(Representation::Circle)
    }
}

#[derive(Debug)]
struct Canvas(Dimensions, Vec<PlottedCoords>);

// -- helper --
fn get_dimensions_from_coords(coords: &Vec<Coords>) -> Dimensions {
    let x = get_max_coord_from_coords(coords.clone().iter(), &|a| a.0);
    let y = get_max_coord_from_coords(coords.clone().iter(), &|a| a.1);

    Dimensions(x + 1, y + 1)
}

fn get_max_coord_from_coords<'a, I: Iterator<Item=&'a Coords>>(coords: I, pluck_fn: &Fn(&Coords) -> u32) -> u32 {
    coords.map(pluck_fn).max().unwrap()
}

// -- functions --
fn combine<T: Plottable + 'static, U: Plottable + 'static>(a: Box<T>, b: Box<U>) -> CombinedObject {
    let mut contained_objects: Vec<Box<Plottable>> = vec![];
    contained_objects.push(a);
    contained_objects.push(b);

    let mut contained_coords = vec![];
    contained_coords.extend(contained_objects[0].get_coords().iter());
    contained_coords.extend(contained_objects[1].get_coords().iter());

    let dimensions = get_dimensions_from_coords(&contained_coords);
    CombinedObject(dimensions, contained_objects)
}

fn calc_coords_from_dimensions(dimensions: Dimensions) -> Vec<Coords> {
    fn coords_from_index(i: u32, dimensions: Dimensions) -> Coords {
        if i < dimensions.0 { Coords(i, 0) }
        else { Coords(i % dimensions.0, i / dimensions.0) }
    };

    (0..(dimensions.0 * dimensions.1))
        .map(|i| { coords_from_index(i, dimensions) })
        .collect()
}

fn plot(a: Box<Plottable>) -> Canvas {
    let mut canvas_coords = calc_coords_from_dimensions(a.get_dimensions())
        .iter()
        .map(|fill_coords| {
            match a.get_representation_at(*fill_coords) {
                Some(representation) => PlottedCoords(fill_coords.0, fill_coords.1, representation),
                None => PlottedCoords(fill_coords.0, fill_coords.1, Representation::Canvas),
            }
        }).collect::<Vec<_>>();

    canvas_coords.sort_by_key(|&PlottedCoords(x, y, _)| (!y, x));

    Canvas(a.get_dimensions(), canvas_coords)
}

// -- tests --
#[test]
fn test_new_horizontal_line_1() {
    let line = Line::new(Coords(0, 0), Coords(3, 0));

    assert_eq!(Dimensions(4, 1), line.get_dimensions());
    assert_eq!(vec![Coords(0, 0), Coords(1, 0), Coords(2, 0), Coords(3, 0)], line.get_coords());
}

#[test]
fn test_new_horizontal_line_2() {
    let line = Line::new(Coords(1, 1), Coords(3, 1));

    assert_eq!(Dimensions(4, 2), line.get_dimensions());
    assert_eq!(vec![Coords(1, 1), Coords(2, 1), Coords(3, 1)], line.get_coords());
}

#[test]
fn test_new_vertical_line() {
    let line = Line::new(Coords(0, 0), Coords(0, 3));

    assert_eq!(Dimensions(1, 4), line.get_dimensions());
    assert_eq!(vec![Coords(0, 0), Coords(0, 1), Coords(0, 2), Coords(0, 3)], line.get_coords());
}

#[test]
fn test_new_diagonal_line_left_to_right() {
    let line = Line::new(Coords(0, 0), Coords(3, 3));

    assert_eq!(Dimensions(4, 4), line.get_dimensions());
    assert_eq!(vec![Coords(0, 0), Coords(1, 1), Coords(2, 2),  Coords(3, 3)], line.get_coords());
}

#[test]
fn test_new_diagonal_line_right_to_left() {
    let line = Line::new(Coords(0, 3), Coords(3, 0));

    assert_eq!(Dimensions(4, 4), line.get_dimensions());
    assert_eq!(vec![Coords(0, 3), Coords(1, 2), Coords(2, 1),  Coords(3, 0)], line.get_coords());
}

#[test]
fn test_get_dimension_from_coords() {
    let coords = vec![Coords(0, 0), Coords(2, 1), Coords(5, 9), Coords(10, 5)];
    let dimensions = get_dimensions_from_coords(&coords);

    assert_eq!(11, dimensions.0);
    assert_eq!(10, dimensions.1);
}

#[test]
fn test_combine_expands_dimensions_to_fit_largest_object_line() {
    // diagonal line, left bottom to top right
    let line_1_coords = [Coords(0, 0), Coords(1, 1), Coords(2, 2)];
    let line_1 = Line(Dimensions(3, 3),  line_1_coords.to_vec());

    // horizontal line, bottom left to bottom right
    let line_2_coords = [Coords(0, 0), Coords(1, 0), Coords(2, 0)];
    let line_2 = Line(Dimensions(3, 1),  line_2_coords.to_vec());

    // combined object is supposed to be large enough to contain 2 lines
    let lines_combined = combine(Box::new(line_1), Box::new(line_2));

    assert_eq!((lines_combined.0).0, 3);
    assert_eq!((lines_combined.0).1, 3);

    assert_eq!(lines_combined.1.len(), 2);
}

#[test]
fn test_combine_expands_dimensions_to_fit_largest_object_circle() {
    // diagonal line, left bottom to top right
    let line_1_coords = [Coords(0, 0), Coords(1, 1), Coords(2, 2)];
    let line_1 = Line(Dimensions(3, 3),  line_1_coords.to_vec());

    // circle with radius 1
    let circle_1_coords = [Coords(1, 2), Coords(0, 1), Coords(1, 0), Coords(2, 1)];
    let circle_1 = Circle(Dimensions(3, 3),  circle_1_coords.to_vec());

    // combined object is supposed to be large enough to contain 2 lines
    let lines_combined = combine(Box::new(line_1), Box::new(circle_1));

    assert_eq!((lines_combined.0).0, 3);
    assert_eq!((lines_combined.0).1, 3);

    assert_eq!(lines_combined.1.len(), 2);
}

#[test]
fn test_plot_merged_object() {
    // diagonal line, left bottom to top right
    let line_1_coords = [Coords(0, 0), Coords(1, 1), Coords(2, 2)];
    let line_1 = Line(Dimensions(3, 3),  line_1_coords.to_vec());

    // horizontal line, bottom left to bottom right
    let line_2_coords = [Coords(0, 0), Coords(1, 0), Coords(2, 0)];
    let line_2 = Line(Dimensions(3, 1),  line_2_coords.to_vec());

    // vertical line, bottom left to top left
    let line_3_coords = [Coords(0, 0), Coords(0, 1), Coords(0, 2), Coords(0, 3)];
    let line_3 = Line(Dimensions(1, 4),  line_3_coords.to_vec());

    // combined object is supposed to be large enough to contain 2 lines
    let lines_combined_1 = combine(Box::new(line_1), Box::new(line_2));
    let lines_combined_2 = combine(Box::new(lines_combined_1), Box::new(line_3));

    let canvas = plot(Box::new(lines_combined_2));

    assert_eq!((canvas.0).0, 3);
    assert_eq!((canvas.0).1, 4);

    assert_eq!((canvas.1).len(), 12);

    assert_eq!(canvas.1[0], PlottedCoords(0, 3, Representation::Line));
    assert_eq!(canvas.1[1], PlottedCoords(1, 3, Representation::Canvas));
    assert_eq!(canvas.1[2], PlottedCoords(2, 3, Representation::Canvas));

    assert_eq!(canvas.1[3], PlottedCoords(0, 2, Representation::Line));
    assert_eq!(canvas.1[4], PlottedCoords(1, 2, Representation::Canvas));
    assert_eq!(canvas.1[5], PlottedCoords(2, 2, Representation::Line));

    assert_eq!(canvas.1[6], PlottedCoords(0, 1, Representation::Line));
    assert_eq!(canvas.1[7], PlottedCoords(1, 1, Representation::Line));
    assert_eq!(canvas.1[8], PlottedCoords(2, 1, Representation::Canvas));

    assert_eq!(canvas.1[9], PlottedCoords(0, 0, Representation::Line));
    assert_eq!(canvas.1[10], PlottedCoords(1, 0, Representation::Line));
    assert_eq!(canvas.1[11], PlottedCoords(2, 0, Representation::Line));
}

#[test]
fn test_plot_line() {
    // diagonal line, left bottom to top right
    let line_1_coords = [Coords(0, 0), Coords(1, 1), Coords(2, 2)];
    let line_1 = Line(Dimensions(3, 3),  line_1_coords.to_vec());

    // horizontal line, bottom left to bottom right
    let line_2_coords = [Coords(0, 0), Coords(1, 0), Coords(2, 0)];
    let line_2 = Line(Dimensions(3, 1),  line_2_coords.to_vec());

    // combined object is supposed to be large enough to contain 2 lines
    let lines_combined = combine(Box::new(line_1), Box::new(line_2));
    let canvas = plot(Box::new(lines_combined));

    assert_eq!((canvas.0).0, 3);
    assert_eq!((canvas.0).1, 3);

    assert_eq!((canvas.1).len(), 9);

    assert_eq!(canvas.1[0], PlottedCoords(0, 2, Representation::Canvas));
    assert_eq!(canvas.1[1], PlottedCoords(1, 2, Representation::Canvas));
    assert_eq!(canvas.1[2], PlottedCoords(2, 2, Representation::Line));

    assert_eq!(canvas.1[3], PlottedCoords(0, 1, Representation::Canvas));
    assert_eq!(canvas.1[4], PlottedCoords(1, 1, Representation::Line));
    assert_eq!(canvas.1[5], PlottedCoords(2, 1, Representation::Canvas));

    assert_eq!(canvas.1[6], PlottedCoords(0, 0, Representation::Line));
    assert_eq!(canvas.1[7], PlottedCoords(1, 0, Representation::Line));
    assert_eq!(canvas.1[8], PlottedCoords(2, 0, Representation::Line));
}

#[test]
fn test_plot_circle() {
    // diagonal line, left bottom to top right
    let line_1_coords = [Coords(0, 0), Coords(1, 1), Coords(2, 2)];
    let line_1 = Line(Dimensions(3, 3),  line_1_coords.to_vec());

    // circle with radius 1
    let circle_1_coords = [Coords(1, 2), Coords(0, 1), Coords(1, 0), Coords(2, 1)];
    let circle_1 = Circle(Dimensions(3, 3),  circle_1_coords.to_vec());

    // combined object is supposed to be large enough to contain 2 lines
    let lines_combined = combine(Box::new(line_1), Box::new(circle_1));
    let canvas = plot(Box::new(lines_combined));

    assert_eq!((canvas.0).0, 3);
    assert_eq!((canvas.0).1, 3);

    assert_eq!((canvas.1).len(), 9);

    assert_eq!(canvas.1[0], PlottedCoords(0, 2, Representation::Canvas));
    assert_eq!(canvas.1[1], PlottedCoords(1, 2, Representation::Circle));
    assert_eq!(canvas.1[2], PlottedCoords(2, 2, Representation::Line));

    assert_eq!(canvas.1[3], PlottedCoords(0, 1, Representation::Circle));
    assert_eq!(canvas.1[4], PlottedCoords(1, 1, Representation::Line));
    assert_eq!(canvas.1[5], PlottedCoords(2, 1, Representation::Circle));

    assert_eq!(canvas.1[6], PlottedCoords(0, 0, Representation::Line));
    assert_eq!(canvas.1[7], PlottedCoords(1, 0, Representation::Circle));
    assert_eq!(canvas.1[8], PlottedCoords(2, 0, Representation::Canvas));
}

fn main () {

}
