use rand::{seq::SliceRandom, rng};
use super::algorithm::{Algorithm, ExecuteResponse};
use crate::models::city::City;

#[derive(Debug, Clone)]
struct Chromossome {
    path: Vec<u16>,
    initial_path: Vec<u16>,
    distance: f64
}

impl Chromossome {
    fn new(path: Vec<u16>, distance: f64) -> Self {
        Chromossome { initial_path: path.clone(), path, distance }
    }

    fn get_path(&self) -> &Vec<u16>{
        &self.path
    }

    fn get_initial_path(&self) -> &Vec<u16> {
        &self.initial_path
    }

    fn get_distance(&self) -> &f64 {
        &self.distance
    }
}

pub struct Genetic;

impl Genetic {
    fn create_random_population(n: usize, cities_n: usize, distance_matrix: &[f64]) -> Vec<Chromossome> {
        let mut population: Vec<Chromossome> = Vec::with_capacity(n);
        let mut current_path: Vec<u16> = (0..=cities_n-1).map(|x| x as u16).collect();
        let mut rng = rng(); 
        
        while population.len() < n {
            population.push(
                Chromossome::new(
                    current_path.clone(), 
                    Self::calculate_path_distance(&current_path, distance_matrix)
                )
            );
            current_path.shuffle(&mut rng);
        }

        population
    }

    fn find_shortest_path_in_population(population: &[Chromossome]) -> Chromossome {
        population
            .iter()
            .min_by(|a, b| a.get_distance().partial_cmp(b.get_distance()).unwrap())
            .unwrap()
            .clone()
    }
}

impl Algorithm for Genetic {
    fn execute(cities: &Vec<City>) -> ExecuteResponse {
        println!("Execute Genetic");
        let population_size: usize = 10;
        let distance_matrix = Self::create_distance_matrix(&cities);
        // println!("{:#?}", distance_matrix);
        let population = Self::create_random_population(population_size, cities.len(), &distance_matrix);
        let best_chromossome = Self::find_shortest_path_in_population(&population);

        ExecuteResponse::new(best_chromossome.get_initial_path().clone(), vec![], best_chromossome.get_distance().clone())
    }
}