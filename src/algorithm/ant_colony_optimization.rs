use std::time::Instant;

use super::algorithm::{Algorithm, ExecuteResponse};
use crate::models::city::City;

pub struct AntColonyOptimization {
    cities: Vec<City>,
}

impl AntColonyOptimization {
    pub fn new(cities: &Vec<City>) -> Self {
        AntColonyOptimization {
            cities: cities.clone(),
        }
    }
}

impl Algorithm for AntColonyOptimization {
    fn execute(&mut self) -> ExecuteResponse {
        // TBD
        println!("Execute AntColonyOptimization");
        ExecuteResponse::new(vec![], vec![], 0.0, Instant::now().elapsed(), String::new())
    }
}
