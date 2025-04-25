use super::algorithm::{Algorithm, ExecuteResponse};
use super::ant_colony_optimization::AntColonyOptimization;
use super::branch_and_bound::BranchAndBound;
use super::genetic::Genetic;
use super::simulated_annealing::SimulatedAnnealing;
use crate::models::city::City;

pub struct AlgorithmStrategy;

impl AlgorithmStrategy {
    pub fn execute_algorithm(algorithm: &str, cities: &Vec<City>) -> ExecuteResponse {
        match algorithm.to_uppercase().as_str() {
            "ACO" => AntColonyOptimization::new(cities).execute(),
            "BB" => BranchAndBound::new(cities).execute(),
            "G" => Genetic::new(cities).execute(),
            "SA" => SimulatedAnnealing::new(cities).execute(),
            _ => panic!("Unknown algorithm")
        }
    }
}
