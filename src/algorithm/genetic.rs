use super::algorithm::{Algorithm, ExecuteResponse};
use crate::models::city::City;

pub struct Genetic;

impl Algorithm for Genetic {
    fn execute(cities: &Vec<City>) -> ExecuteResponse {
        println!("Execute Genetic");
        let distance_matrix = Self::create_distance_matrix(&cities);
        let initial_path: Vec<u16> = (0..=cities.len()-1).map(|x| x as u16).collect();
        ExecuteResponse::new(initial_path, vec![], 0.0)

    }
}