use super::algorithm::Algorithm;
use crate::models::city::City;

pub struct SimulatedAnnealing;

impl Algorithm for SimulatedAnnealing {
    fn execute(cities: Vec<City>) -> Vec<City> {
        // TBD
        println!("Execute SimulatedAnnealing");
        cities
    }
}