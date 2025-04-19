use super::algorithm::Algorithm;
use crate::models::city::City;

pub struct AntColonyOptimization;

impl Algorithm for AntColonyOptimization {
    fn execute(cities: Vec<City>) -> Vec<City> {
        // TBD
        println!("Execute AntColonyOptimization");
        cities
    }
}