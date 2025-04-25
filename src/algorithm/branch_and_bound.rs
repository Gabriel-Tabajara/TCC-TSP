use std::time::Instant;

use super::algorithm::{Algorithm, ExecuteResponse};
use crate::models::city::City;

pub struct BranchAndBound;

impl Algorithm for BranchAndBound {
    fn execute(cities: &Vec<City>) -> ExecuteResponse {
        // TBD
        println!("Execute BranchAndBound");
        ExecuteResponse::new(vec![], vec![], 0.0, Instant::now().elapsed(), String::new())
    }
}