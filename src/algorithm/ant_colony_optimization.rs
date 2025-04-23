use std::time::Instant;

use super::algorithm::{Algorithm, ExecuteResponse};
use crate::models::city::City;

pub struct AntColonyOptimization;

impl Algorithm for AntColonyOptimization {
    fn execute(cities: &Vec<City>) -> ExecuteResponse {
        // TBD
        println!("Execute AntColonyOptimization");
        ExecuteResponse::new(vec![], vec![], 0.0, Instant::now().elapsed())
    }
}