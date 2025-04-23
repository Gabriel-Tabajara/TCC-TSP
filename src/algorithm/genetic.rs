use std::time::Instant;

use rand::{rng, seq::SliceRandom, Rng};
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

    fn sort_population_by_distance(population: &[Chromossome]) -> Vec<Chromossome> {
        let mut sorted = population.to_vec();
        sorted.sort_by(|a: &Chromossome, b| a.distance.partial_cmp(&b.distance).unwrap());
        sorted
    }

    fn select_parents(population: &[Chromossome]) -> (Chromossome, Chromossome) {
        let mut rng = rng(); 
        let mut shuffle_result = population.to_vec();
        shuffle_result.shuffle(&mut rng);
        (shuffle_result[0].clone(), shuffle_result[1].clone())
    }

    fn single_point_crossover(parent_1: &Chromossome, parent_2: &Chromossome, distance_matrix: &[f64]) -> Chromossome{
        let mut rng = rng();
        let n = parent_1.get_path().len();
        let mut div_point = rng.random_range(1..n-2);
        let parent_1_part = &parent_1.get_path()[..div_point];
        // let parent_2_part = &parent_2.get_path()[div_point..];

        let mut path = parent_1_part.to_vec();
        let parent_2_path = &parent_2.get_path();
        while path.len() < n {
            let id = parent_2_path[div_point];
            if !path.contains(&id) {
                path.push(id);
            }
            if div_point >= parent_2_path.len() - 1 {
                div_point = 0;
            } else {
                div_point += 1;
            }
        }

        let distance = Self::calculate_path_distance(&path, &distance_matrix);
        Chromossome::new(path, distance)
    }

    fn displacement_mutation(chromossome: &mut Chromossome, size: usize) {
        let mut rng = rng();
        if rng.random_bool(0.3) {
            let n = chromossome.get_path().len();
            let div: usize = rng.random_range(1..n-size-1);
            let mut path = chromossome.get_path().clone();
            let sub_array: Vec<u16> = path.splice(div..div + size, []).collect::<Vec<u16>>();
            path.splice(div + size..div + size, sub_array);
        }
    }
    
    fn add_chromossome_to_sorted_population(chromossome: Chromossome, sorted_population: &mut Vec<Chromossome>) {
        let worst_n = sorted_population.len() - 1;
        let worst_chromossome = &sorted_population[worst_n];
        if chromossome.get_distance() < worst_chromossome.get_distance() {
            println!("{}", &chromossome.get_distance());
            sorted_population.remove(worst_n);
            sorted_population.push(chromossome);
            sorted_population.sort_by(|a: &Chromossome, b| a.distance.partial_cmp(&b.distance).unwrap());
        }
    }
}

impl Algorithm for Genetic {
    fn execute(cities: &Vec<City>) -> ExecuteResponse {
        println!("Execute Genetic");
        let start_time = Instant::now();
        let population_size: usize = 10;
        let cities_size: usize = cities.len()/10;
        let distance_matrix = Self::create_distance_matrix(&cities);

        let population = Self::create_random_population(population_size, cities.len(), &distance_matrix);
        let mut sorted_population = Self::sort_population_by_distance(&population);

        let mut gen_max = 1000;
        
        while gen_max > 0 {
            // let (parent_1, parent_2) = Self::select_parents(&sorted_population);
            let (parent_1, parent_2) = (sorted_population[0].clone(), sorted_population[1].clone());
            let mut children = Self::single_point_crossover(&parent_1, &parent_2, &distance_matrix);
            Self::displacement_mutation(&mut children, cities_size);
            Self::add_chromossome_to_sorted_population(children, &mut sorted_population);
            gen_max -= 1;
        }

        let total_time = start_time.elapsed();
        ExecuteResponse::new(sorted_population[0].get_initial_path().clone(), vec![], sorted_population[0].get_distance().clone(), total_time)
    }
}