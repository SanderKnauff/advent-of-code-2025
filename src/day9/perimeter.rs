use crate::day9::geometry_primitives::{Coordinate, Edge, Outlined};
use crate::day9::rectangle::Rectangle;

#[derive(Debug, Clone)]
pub struct Perimeter {
    pub edges: Vec<Edge>,
}

//...............
// .......#XXX#..
// .......X...X..
// ..#XXXX#...X..
// ..X........X..
// ..#XXXXXX#.X..
// .........X.X..
// .........#X#..
// ..............
impl Perimeter {
    pub fn contains(&self, rectangle: &Rectangle) -> bool {
        // All corners must be inside perimeter
        let are_corners_valid = rectangle.calculate_corners()
            .iter()
            .all(|corner| self.contains_coordinate(corner));

        if !are_corners_valid {
            return false;
        }

        // None of the edges of the rectangle can cross an edge of the perimeter
        for perimeter_edge in &self.edges {
            for rectangle_edge in rectangle.get_edges() {
                if perimeter_edge.intersects_with(&rectangle_edge) {
                    return false;
                }
            }
        }

        true
    }

    pub fn contains_coordinate(&self, coordinate: &Coordinate) -> bool {
        false
    }
}

impl Outlined for Perimeter {
    fn get_edges(&self) -> Vec<Edge> {
        self.edges.clone()
    }
}
