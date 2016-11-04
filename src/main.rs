//use std::collections::HashSet;
// @TODO wanna use HashSet as coords can only be unique, see
// http://stackoverflow.com/questions/27828487/hashmap-with-hashset-as-key
// (need to implement Hash for Hashmap ...)


#[derive(Debug, PartialEq, Clone)]
enum Representation {
    Canvas,
    Line,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Coords(u32, u32);

#[derive(Debug, PartialEq, Clone)]
struct PlottedCoords(u32, u32, Representation);

#[derive(Debug, PartialEq, Copy, Clone)]
struct Dimensions(u32, u32);

#[derive(Debug)]
struct Line(Dimensions, Vec<Coords>);

#[derive(Debug)]
struct CombinedObject(Dimensions, Vec<Line>, Vec<Coords>);

#[derive(Debug)]
struct Canvas(Dimensions, Vec<PlottedCoords>);

// -- helper --
fn get_max_coord_from_coords(coords: std::slice::Iter<Coords>, pluck_fn: &Fn(&Coords) -> u32) -> u32 {
    coords.map(pluck_fn).max().unwrap()
}

fn get_max_dimensions_from_coords(coords: std::slice::Iter<Coords>) -> Dimensions {
    let max_x = get_max_coord_from_coords(coords.clone(), &|coords| coords.0);
    let max_y = get_max_coord_from_coords(coords.clone(), &|coords| coords.0);

    Dimensions(max_x + 1, max_y + 1)
}

// -- functions --
fn combine(a: Line, b: Line) -> CombinedObject {
    let mut contained_objects = Vec::new();
    contained_objects.push(a);
    contained_objects.push(b);

    let mut contained_coords = Vec::new();
    contained_coords.extend(contained_objects[0].1.iter());
    contained_coords.extend(contained_objects[1].1.iter());

    let dimensions = get_max_dimensions_from_coords(contained_coords.iter());
    CombinedObject(dimensions, contained_objects, contained_coords)
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

fn plot(a: CombinedObject) -> Canvas {
    let mut canvas_coords = calc_coords_from_dimensions(a.0).iter().map(|fill_coords| {
        match a.2.iter().find(|combined_coords| **combined_coords == *fill_coords) {
            Some(found_combined_coords) => PlottedCoords(found_combined_coords.0, found_combined_coords.1, Representation::Line),
            None => PlottedCoords(fill_coords.0, fill_coords.1, Representation::Canvas),
        }
    }).collect::<Vec<_>>();

    canvas_coords.sort_by_key(|&PlottedCoords(x, y, _)| (!y, x));

    Canvas(a.0, canvas_coords)
}

// -- tests --
#[test]
fn test_combine_expands_dimensions_to_fit_largest_object() {
    // diagonal line, left bottom to top right
    let line_1_coords = [Coords(0, 0), Coords(1, 1), Coords(2, 2)];
    let line_1 = Line(Dimensions(3, 3),  line_1_coords.to_vec());

    // horizontal line, bottom left to bottom right
    let line_2_coords = [Coords(0, 0), Coords(1, 0), Coords(2, 0)];
    let line_2 = Line(Dimensions(3, 1),  line_2_coords.to_vec());

    // combined object is supposed to be large enough to contain 2 lines
    let lines_combined = combine(line_1, line_2);

    assert_eq!((lines_combined.0).0, 3);
    assert_eq!((lines_combined.0).1, 3);

    assert_eq!(lines_combined.1.len(), 2);
}

#[test]
fn test_plot() {
    // diagonal line, left bottom to top right
    let line_1_coords = [Coords(0, 0), Coords(1, 1), Coords(2, 2)];
    let line_1 = Line(Dimensions(3, 3),  line_1_coords.to_vec());

    // horizontal line, bottom left to bottom right
    let line_2_coords = [Coords(0, 0), Coords(1, 0), Coords(2, 0)];
    let line_2 = Line(Dimensions(3, 1),  line_2_coords.to_vec());

    // combined object is supposed to be large enough to contain 2 lines
    let lines_combined = combine(line_1, line_2);
    let canvas = plot(lines_combined);

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

fn main () {

}
