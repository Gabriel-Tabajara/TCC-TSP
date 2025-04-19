use super::algorithm::Algorithm;
use super::ant_colony_optimization::AntColonyOptimization;
use super::branch_and_bound::BranchAndBound;
use super::genetic::Genetic;
use super::simulated_annealing::SimulatedAnnealing;
use crate::models::city::City;

pub struct AlgorithmStrategy;

impl AlgorithmStrategy {
    pub fn execute_algorithm(algorithm: &str, cities: Vec<City>) -> Vec<City> {
        match algorithm {
            "AntColonyOptimization" => AntColonyOptimization::execute(cities),
            "BranchAndBound" => BranchAndBound::execute(cities),
            "Genetic" => Genetic::execute(cities),
            "SimulatedAnnealing" => SimulatedAnnealing::execute(cities),
            _ => panic!("Unknown algorithm")
        }
    }
}
