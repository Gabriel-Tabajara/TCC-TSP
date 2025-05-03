use std::time::Instant;

use super::algorithm::{Algorithm, ExecuteResponse};
use crate::models::city::City;

pub struct SimulatedAnnealing {
    cities: Vec<City>,
}

impl SimulatedAnnealing {
    pub fn new(cities: &Vec<City>) -> Self {
        SimulatedAnnealing {
            cities: cities.clone(),
        }
    }
}
impl Algorithm for SimulatedAnnealing {
    fn execute(&mut self) -> ExecuteResponse {
        // TBD
        println!("Execute SimulatedAnnealing");
        ExecuteResponse::new(vec![], vec![], 0.0, Instant::now().elapsed(), String::new())
    }
}
