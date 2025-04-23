use std::time::Instant;

use super::algorithm::{Algorithm,ExecuteResponse};
use crate::models::city::City;

pub struct SimulatedAnnealing;

impl Algorithm for SimulatedAnnealing {
    fn execute(cities: &Vec<City>) -> ExecuteResponse {
        // TBD
        println!("Execute SimulatedAnnealing");
        ExecuteResponse::new(vec![], vec![], 0.0, Instant::now().elapsed())
    }
}