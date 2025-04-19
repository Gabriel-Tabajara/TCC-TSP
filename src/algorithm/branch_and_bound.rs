use super::algorithm::Algorithm;
use crate::models::city::City;

pub struct BranchAndBound;

impl Algorithm for BranchAndBound {
    fn execute(cities: Vec<City>) -> Vec<City> {
        // TBD
        println!("Execute BranchAndBound");
        cities
    }
}