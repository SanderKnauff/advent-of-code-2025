use std::cmp::{max, min};
use crate::day9::geometry_primitives::{Coordinate, Edge, Outlined};

#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    pub first_corner: Coordinate,
    pub second_corner: Coordinate,
}

impl Outlined for Rectangle {
    fn get_edges(&self) -> Vec<Edge> {
        let mut edges: Vec<Edge> = Vec::new();
        let corners = self.calculate_corners();

        for (index, first_corner) in corners.iter().enumerate() {
            for second_corner in corners[index..].iter() {
                let edge = Edge::new(first_corner, second_corner).unwrap_or_else(|err| panic!("{}", err));
                edges.push(edge);
            }
        }

        edges
    }
}

impl Rectangle {
    pub fn calculate_area(&self) -> u64 {
        self.first_corner.calculate_area(&self.second_corner)
    }

    pub fn calculate_corners(&self) -> Vec<Coordinate> {
        let top_left_corner = Coordinate {
            x: min(self.first_corner.x, self.second_corner.x),
            y: min(self.first_corner.y, self.second_corner.y),
        };
        let top_right_corner = Coordinate {
            x: max(self.first_corner.x, self.second_corner.x),
            y: min(self.first_corner.y, self.second_corner.y),
        };
        let bottom_left_corner = Coordinate {
            x: min(self.first_corner.x, self.second_corner.x),
            y: max(self.first_corner.y, self.second_corner.y),
        };
        let bottom_right_corner = Coordinate {
            x: max(self.first_corner.x, self.second_corner.x),
            y: max(self.first_corner.y, self.second_corner.y),
        };

        Vec::from([
            top_left_corner,
            top_right_corner,
            bottom_left_corner,
            bottom_right_corner,
        ])
    }
}
