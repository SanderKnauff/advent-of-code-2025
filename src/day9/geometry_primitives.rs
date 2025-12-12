use std::error::Error;
use crate::day9::geometry_primitives::Direction::{Horizontal, Vertical};

#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    pub x: u64,
    pub y: u64,
}

impl Coordinate {
    pub fn calculate_area(&self, other: &Coordinate) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

#[derive(PartialEq, Eq)]
enum Direction {
    Horizontal,
    Vertical
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub start: Coordinate,
    pub end: Coordinate,
}

impl Edge {
    pub fn new(start: &Coordinate, end: &Coordinate) -> Result<Edge, Box<dyn Error>> {
        if start.x != end.x && start.y != end.y {
            Err(format!("{start:?} and {end:?} do not exist on a single orthogonal line. Either the X or the Y of the start and end must match."))?;
        }

        Ok(Edge {
            start: *start,
            end: *end,
        })
    }

    fn contains_coordinate(&self, coordinate: &Coordinate) {

    }

    fn direction(&self) -> Direction {
        if self.start.x == self.end.x {
            Horizontal
        } else if self.start.y == self.end.y {
            Vertical
        } else {
            panic!("Invalid direction for Edge {self:?}")
        }
    }
}

pub trait Outlined {
    fn get_edges(&self) -> Vec<Edge>;
}