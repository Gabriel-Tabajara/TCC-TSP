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

    fn get_mut_path(&mut self) -> &mut Vec<u16>{
        &mut self.path
    }

    fn get_initial_path(&self) -> &Vec<u16> {
        &self.initial_path
    }

    fn get_distance(&self) -> &f64 {
        &self.distance
    }

    fn swap_mutation(mut self, distance_matrix: &[f64], swaps: usize) -> Self {
        let mut rng = rng();
        let n = &self.path.len();
        let mut path = self.path.clone();
        for i in 0..swaps {     
            let first = rng.random_range(0..n-1);
            let second = rng.random_range(0..n-1);
            
            path.swap(first, second);
        }
        let updated_distance = Genetic::calculate_path_distance(&path, distance_matrix);
        if &updated_distance < &self.distance {
            self.distance = updated_distance;
            self.path = path;
        }
        self
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

    fn order_crossover(parent_1: &Chromossome, parent_2: &Chromossome, distance_matrix: &[f64]) -> Chromossome {
        let mut rng = rng();
        let n = parent_1.get_path().len();

        let start = rng.random_range(0..n-2);
        let end = rng.random_range(start+1..n-1);

        let mut path = vec![0; n];
        path[start..end].copy_from_slice(&parent_1.get_path()[start..end]);

        let mut index = end;
        for &city in parent_2.get_path() {
            if !path.contains(&city) {
                path[index % n] = city;
                index += 1;
            }
        }

        let distance = Self::calculate_path_distance(&path, &distance_matrix);
        Chromossome::new(path, distance)
    }
    
    fn add_chromossome_to_sorted_population(chromossome: Chromossome, sorted_population: &mut Vec<Chromossome>) {
        let worst_n = sorted_population.len() - 1;
        let worst_chromossome = &sorted_population[worst_n];
        if chromossome.get_distance() < worst_chromossome.get_distance() {
            sorted_population.remove(worst_n);
            sorted_population.push(chromossome);
            sorted_population.sort_by(|a: &Chromossome, b| a.distance.partial_cmp(&b.distance).unwrap());
        }
    }
    
    fn execute_for_population(mut sorted_population: Vec<Chromossome>, distance_matrix: &[f64], cities_len: usize, gen_max_init: i32, population_size: usize, initial_swap: usize) -> Vec<Chromossome> {
        let mut previous_distance = sorted_population[0].get_distance().clone();
        let mut gen_max_iter = gen_max_init;
        let mut gen_not_changed_best = 0;
        let mut gen_not_changed_best_limit = cities_len;
        let mut swap = initial_swap;

        while gen_max_iter > 0 {
            // let (parent_1, parent_2) = (sorted_population[0].clone(), sorted_population[1].clone());
            let (parent_1, parent_2) = Self::select_parents(&sorted_population);
            let children = Self::order_crossover(&parent_1, &parent_2, &distance_matrix);
            sorted_population[population_size-1] = children.clone();
            for i in 2..population_size-2 {
                sorted_population[i] = children.clone().swap_mutation(&distance_matrix, swap);
                if swap > 1 {
                    swap -= 1;
                }
            }
            sorted_population = Self::sort_population_by_distance(&sorted_population);
            let best_distance = sorted_population[0].get_distance();
            if previous_distance > *best_distance {
                previous_distance = best_distance.clone();
                println!("{} {}", &best_distance, gen_max_init - gen_max_iter);
            } else {
                gen_not_changed_best += 1;
            }
            if gen_not_changed_best > gen_not_changed_best_limit {
                gen_not_changed_best_limit *= 2;
                if swap < cities_len {
                    swap += 1;
                } else {
                    swap = 1
                }
            }
            gen_max_iter -= 1;
        }

        sorted_population
    }

    fn execute_for_one_population_army(first_gen: &Chromossome, distance_matrix: &[f64], cities_len: usize, gen_max_init: i32,initial_swap: usize) -> Chromossome {
        let mut previous_distance = first_gen.get_distance().clone();
        let mut gen_max_iter = gen_max_init;
        let mut gen_not_changed_best = 0;
        let mut gen_not_changed_best_limit = cities_len;
        let mut swap = initial_swap;
        let mut current_gen = first_gen.clone();

        while gen_max_iter > 0 {
            let new_gen = current_gen.clone().swap_mutation(distance_matrix, swap);
            let new_distance = new_gen.get_distance();
            if previous_distance > *new_distance {
                previous_distance = new_distance.clone();
                current_gen = new_gen.clone();
                println!("{} {}", &new_distance, gen_max_init - gen_max_iter);
            } else {
                gen_not_changed_best += 1;
            }
            if gen_not_changed_best > gen_not_changed_best_limit {
                gen_not_changed_best_limit *= 2;
                if swap < cities_len {
                    swap += 1;
                } else {
                    swap = 1
                }
            }
            gen_max_iter -= 1;
        }
        current_gen
    }
}

impl Algorithm for Genetic {
    fn execute(cities: &Vec<City>) -> ExecuteResponse {
        println!("Execute Genetic");
        let start_time = Instant::now();
        let population_size: usize = 5;
        let distance_matrix = Self::create_distance_matrix(&cities);
        
        let population = Self::create_random_population(population_size, cities.len(), &distance_matrix);
        let mut sorted_population = Self::sort_population_by_distance(&population);

        let swap = 1;
        let gen_max_init = 500000;
    
        if population_size == 1 {
            sorted_population[0] = Self::execute_for_one_population_army(&sorted_population[0], &distance_matrix, cities.len(), gen_max_init, swap);
        } else {
            sorted_population = Self::execute_for_population(sorted_population, &distance_matrix, cities.len(), gen_max_init, population_size, swap);
        }

        ExecuteResponse::new(sorted_population[0].get_initial_path().clone(), sorted_population[0].get_path().clone(), sorted_population[0].get_distance().clone(), start_time.elapsed())
    }
}