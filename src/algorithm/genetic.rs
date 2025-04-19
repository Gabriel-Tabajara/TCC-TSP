use super::algorithm::Algorithm;
use crate::models::city::City;

pub struct Genetic;

impl Algorithm for Genetic {
    fn execute(cities: Vec<City>) -> Vec<City> {
        // TBD
        println!("Execute Genetic");
        cities
    }
}