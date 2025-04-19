use super::algorithm::Algorithm;
use super::ant_colony_optimization::AntColonyOptimization;
use super::branch_and_bound::BranchAndBound;
use super::genetic::Genetic;
use super::simulated_annealing::SimulatedAnnealing;
use crate::models::city::City;

pub struct AlgorithmStrategy;

impl AlgorithmStrategy {
    pub fn execute_algorithm(algorithm: &str, cities: Vec<City>) -> Vec<City> {
        match algorithm.to_uppercase().as_str() {
            "ACO" => AntColonyOptimization::execute(cities),
            "BB" => BranchAndBound::execute(cities),
            "G" => Genetic::execute(cities),
            "SA" => SimulatedAnnealing::execute(cities),
            _ => panic!("Unknown algorithm")
        }
    }
}
