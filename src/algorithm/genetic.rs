use std::time::Instant;

use rand::{rng, seq::SliceRandom, Rng};
use super::algorithm::{Algorithm, ExecuteResponse};
use crate::models::city::City;

#[derive(Debug, Clone)]
struct Chromossome {
    path: Vec<u16>,
    distance: f64
}

impl Chromossome {
    fn new(path: Vec<u16>, distance: f64) -> Self {
        Chromossome { path, distance }
    }

    fn get_path(&self) -> &Vec<u16>{
        &self.path
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

pub struct Genetic {
    distance_matrix: Vec<f64>,
    cities: Vec<City>,
    crossover: String,
    mutations: Vec<String>
}

impl Genetic {
    pub fn new(cities: &Vec<City>) -> Self {
        Genetic { 
            distance_matrix: vec![], 
            cities: cities.clone(),
            crossover: String::new(),
            mutations: vec![]
        }
    }

    fn create_random_population(&self, n: usize) -> Vec<Chromossome> {
        let mut population: Vec<Chromossome> = Vec::with_capacity(n);
        let mut current_path: Vec<u16> = (0..=self.cities.len()-1).map(|x| x as u16).collect();
        let mut rng = rng(); 
        
        while population.len() < n {
            population.push(
                Chromossome::new(
                    current_path.clone(), 
                    Self::calculate_path_distance(&current_path, &self.distance_matrix)
                )
            );
            current_path.shuffle(&mut rng);
        }

        population
    }

    fn sort_population_by_distance(&self, population: &[Chromossome]) -> Vec<Chromossome> {
        let mut sorted = population.to_vec();
        sorted.sort_by(|a: &Chromossome, b| a.distance.partial_cmp(&b.distance).unwrap());
        sorted
    }

    fn select_parents(&self, population: &[Chromossome]) -> (Chromossome, Chromossome) {
        let mut rng = rng(); 
        let mut shuffle_result = population.to_vec();
        shuffle_result.shuffle(&mut rng);
        (shuffle_result[0].clone(), shuffle_result[1].clone())
    }

    fn order_crossover(&mut self, parent_1: &Chromossome, parent_2: &Chromossome) -> Chromossome {
        self.crossover = "order_crossover".to_string();

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

        let distance = Self::calculate_path_distance(&path, &self.distance_matrix);
        Chromossome::new(path, distance)
    }
    
    fn execute_for_population(&mut self, mut sorted_population: Vec<Chromossome>, gen_max_init: i32, population_size: usize, initial_swap: usize) -> Vec<Chromossome> {
        let mut previous_distance = sorted_population[0].get_distance().clone();
        let mut gen_max_iter = gen_max_init;
        let mut gen_not_changed_best = 0;
        let mut gen_not_changed_best_limit = self.cities.len();
        let mut swap = initial_swap;

        while gen_max_iter > 0 {
            // let (parent_1, parent_2) = (sorted_population[0].clone(), sorted_population[1].clone());
            let (parent_1, parent_2) = self.select_parents(&sorted_population);
            let children = self.order_crossover(&parent_1, &parent_2);
            sorted_population[population_size-1] = children.clone();
            for i in 2..population_size-2 {
                sorted_population[i] = children.clone().swap_mutation(&self.distance_matrix, swap);
                if swap > 1 {
                    swap -= 1;
                }
            }
            sorted_population = self.sort_population_by_distance(&sorted_population);
            let best_distance = sorted_population[0].get_distance();
            if previous_distance > *best_distance {
                previous_distance = best_distance.clone();
                println!("{} {}", &best_distance, gen_max_init - gen_max_iter);
            } else {
                gen_not_changed_best += 1;
            }
            if gen_not_changed_best > gen_not_changed_best_limit {
                gen_not_changed_best_limit *= 2;
                if swap < self.cities.len() {
                    swap += 1;
                } else {
                    swap = 1
                }
            }
            gen_max_iter -= 1;
        }

        sorted_population
    }

    fn execute_for_one_population_army(&self, first_gen: &Chromossome, gen_max_init: i32,initial_swap: usize) -> Chromossome {
        let mut previous_distance = first_gen.get_distance().clone();
        let mut gen_max_iter = gen_max_init;
        let mut gen_not_changed_best = 0;
        let mut gen_not_changed_best_limit = self.cities.len();
        let mut swap = initial_swap;
        let mut current_gen = first_gen.clone();

        while gen_max_iter > 0 {
            let new_gen = current_gen.clone().swap_mutation(&self.distance_matrix, swap);
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
                if swap < self.cities.len() {
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

// TODO: Critério de parada
// TODO: Mais mutações (greedy)
// TODO: Testar outros crossovers
impl Algorithm for Genetic {
    fn execute(&mut self) -> ExecuteResponse {
        println!("Execute Genetic");
        let start_time = Instant::now();
        let population_size: usize = 15;
        self.distance_matrix = Self::create_distance_matrix(&self.cities);
        
        let population = self.create_random_population(population_size);
        let mut sorted_population = self.sort_population_by_distance(&population);

        let first_gen_best_path = sorted_population[0].get_path().clone();

        let swap = 1;
        let gen_max_init = 100000;
    
        if population_size == 1 {
            sorted_population[0] = self.execute_for_one_population_army(&sorted_population[0], gen_max_init, swap);
        } else {
            sorted_population = self.execute_for_population(sorted_population, gen_max_init, population_size, swap);
        }

        let metadata = format!("Population Size: {}\nGenerations: {}\nCrossover: {}\nMutations: {}", 
            population_size,
            gen_max_init,
            self.crossover,
            "swap_mutation"
        );

        ExecuteResponse::new(
            first_gen_best_path, 
            sorted_population[0].get_path().clone(), 
            sorted_population[0].get_distance().clone(), 
            start_time.elapsed(),
            metadata
        )
    }
}