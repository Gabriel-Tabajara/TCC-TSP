use super::algorithm::{Algorithm, ExecuteResponse};
use crate::models::city::City;

pub struct Genetic;

impl Algorithm for Genetic {
    fn execute(cities: &Vec<City>) -> ExecuteResponse {
        println!("Execute Genetic");
        let distance_matrix = Self::create_distance_matrix(&cities);
        // println!("{:#?}", distance_matrix);
        let initial_path: Vec<u16> = (0..=cities.len()-1).map(|x| x as u16).collect();
        let initial_distance = Self::calculate_path_distance(&initial_path, &distance_matrix);
        ExecuteResponse::new(initial_path, vec![], initial_distance)
    }
}