extern crate lib;

use std::f64::consts::SQRT_2;
use lib::PathFindOption;

pub trait PathFindOption: Sized + Copy {}

/// Heuristic function trait
pub trait Heuristic: PathFindOption {
    /// get heuristic function
    fn heuristic(&self, dx: f64, dy: f64) -> f64;
}


/// Manhattan distance.
#[derive(Copy, Clone, PathFindOption)]
pub struct Manhattan;

impl Heuristic for Manhattan {
    fn heuristic(&self, dx: f64, dy: f64) -> f64 {
        dx + dy
    }
}

/// Euclidean distance
#[derive(Copy, Clone,PathFindOption)]
pub struct Euclidean;

impl Heuristic for Euclidean {
    fn heuristic(&self, dx: f64, dy: f64) -> f64 {
        (dx * dx + dy * dy as f64).sqrt()
    }
}

/// Octile distance
#[derive(Copy, Clone,PathFindOption)]
pub struct Octile;

impl Heuristic for Octile {
    fn heuristic(&self, dx: f64, dy: f64) -> f64 {
        if dx < dy { (SQRT_2 - 1.0) * dx + dy } else { (SQRT_2 - 1.0) * dy + dx }
    }
}

/// Chebyshev distance
#[derive(Copy, Clone,PathFindOption)]
pub struct Chebyshev;

impl Heuristic for Chebyshev {
    fn heuristic(&self, dx: f64, dy: f64) -> f64 {
        if dx > dy { dx } else { dy }
    }
}

fn main() {}