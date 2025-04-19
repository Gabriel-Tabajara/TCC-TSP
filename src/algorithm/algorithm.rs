use crate::models::city::City;

pub trait Algorithm {
    fn execute(cities: Vec<City>) -> Vec<City>;
}