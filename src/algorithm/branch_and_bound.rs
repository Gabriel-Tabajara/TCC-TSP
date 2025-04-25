use std::time::Instant;

use super::algorithm::{Algorithm, ExecuteResponse};
use crate::models::city::City;

pub struct BranchAndBound {
    cities: Vec<City>,
}

impl BranchAndBound {
    pub fn new(cities: &Vec<City>) -> Self {
        BranchAndBound { cities: cities.clone() }
    }
}

impl Algorithm for BranchAndBound {
    fn execute(&mut self) -> ExecuteResponse {
        // TBD
        println!("Execute BranchAndBound");
        ExecuteResponse::new(vec![], vec![], 0.0, Instant::now().elapsed(), String::new())
    }
}