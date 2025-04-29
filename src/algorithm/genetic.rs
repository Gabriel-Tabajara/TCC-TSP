use std::{collections::HashSet, time::Instant};

use rand::{rng, seq::SliceRandom, Rng};
use super::algorithm::{Algorithm, ExecuteResponse};
use crate::models::city::City;

#[derive(Debug, Clone)]
struct Chromossome {
    path: Vec<u16>,
    distance: f64,
    mutation: String
}

impl Chromossome {
    fn new(path: Vec<u16>, distance: f64) -> Self {
        Chromossome { path, distance, mutation: "".to_string() }
    }

    fn get_path(&self) -> &Vec<u16>{
        &self.path
    }

    fn get_distance(&self) -> &f64 {
        &self.distance
    }

    fn get_mutation(&self) -> &String {
        &self.mutation
    }

    fn update_distance(mut self, path: Vec<u16>, distance_matrix: &[f64]) -> Self {
        let updated_distance = Genetic::calculate_path_distance(&path, distance_matrix);
        if &updated_distance < &self.distance {
            self.distance = updated_distance;
            self.path = path;
        }
        self
    }

    fn mutate(self, distance_matrix: &[f64], swaps: usize) -> Self {
        let mut rng = rng();
        let swap_mut = rng.random_bool(0.5);
        if swap_mut {
            self.swap_mutation(distance_matrix, swaps)
        } else {
            self.displacement_mutation(distance_matrix)
        }
    }

    fn swap_mutation(mut self, distance_matrix: &[f64], swaps: usize) -> Self {
        self.mutation = "swap_mutation".to_string();
        let mut rng = rng();
        let n = &self.path.len();
        let mut path = self.path.clone();
        for i in 0..swaps {     
            let first = rng.random_range(0..n-1);
            let second = rng.random_range(0..n-1);
            
            path.swap(first, second);
        }
        
        self.update_distance(path, distance_matrix)
    }

    fn displacement_mutation(mut self, distance_matrix: &[f64]) -> Self {
        self.mutation = "displacement_mutation".to_string();
        let mut rng = rng();
        let n = &self.path.len();
        let mut path = self.path.clone();
        let shift_size = rng.random_range(2..n-1);
        let distance2 = rng.random_range(0..n-1);

        let shift_position = rng.random_range(0..n-shift_size-1);
        let displaced_part: Vec<u16> = path.drain(shift_position..shift_position+shift_size).collect();
        
        let new_position = (shift_position + distance2) % (n-shift_size);
        path.splice(new_position..new_position, displaced_part);

        self.update_distance(path, distance_matrix)
    }
}

pub struct Genetic {
    distance_matrix: Vec<f64>,
    cities: Vec<City>,
    crossover: String,
    mutations: HashSet<String>,
    generations: u32
}

impl Genetic {
    pub fn new(cities: &Vec<City>) -> Self {
        Genetic { 
            distance_matrix: vec![], 
            cities: cities.clone(),
            crossover: String::new(),
            mutations: HashSet::new(),
            generations: 0
        }
    }

    fn create_random_population(&self, n: usize) -> Vec<Chromossome> {
        let mut population: Vec<Chromossome> = Vec::with_capacity(n);
        let mut current_path: Vec<u16> = (0..=self.cities.len()-1).map(|x| x as u16).collect();
        let mut rng = rng(); 
        
        while population.len() < n {
            current_path.shuffle(&mut rng);
            population.push(
                Chromossome::new(
                    current_path.clone(), 
                    Self::calculate_path_distance(&current_path, &self.distance_matrix)
                )
            );
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

        let mut start = rng.random_range(0..n-2);
        let mut end = rng.random_range(start+1..n-1);

        let mut path = vec![u16::MAX; n];
        path[start..end].copy_from_slice(&parent_1.get_path()[start..end]);
        for &city in parent_2.get_path() {
            if !path.contains(&city) {
                if end <= n - 1 {
                    path[end] = city;
                    end += 1;
                } else {
                    path[start-1] = city;
                    start -= 1;
                }
            }
        }

        let distance = Self::calculate_path_distance(&path, &self.distance_matrix);
        Chromossome::new(path, distance)
    }

    fn order_based_crossover(&mut self, parent_1: &Chromossome, parent_2: &Chromossome) -> Chromossome { 
        self.crossover = "order_based_crossover".to_string();

        let mut rng = rng();
        let n = parent_1.get_path().len();

        let mut start = rng.random_range(0..n-2);
        let end = rng.random_range(start+1..n-1);
        
        let mut path = parent_1.get_path().clone();
        path.drain(start..end);
        for &city in parent_2.get_path() {
            if !path.contains(&city) {
                path.insert(start, city);
                start += 1;
            }
        }
        
        let distance = Self::calculate_path_distance(&path, &self.distance_matrix);
        Chromossome::new(path, distance)
    }
    
    fn cycle_crossover(&mut self, parent_1: &Chromossome, parent_2: &Chromossome) -> Chromossome { 
        self.crossover = "cycle_crossover".to_string();
        let parent_1_path = parent_1.get_path();
        let parent_2_path = parent_2.get_path();

        let n = parent_1_path.len();
        let mut path = vec![u16::MAX; n];

        let mut current = parent_1_path[0];
        let mut cycle = vec![];
        let mut availiable_pos: Vec<usize> = (0..n).collect();

        // println!("\n{:?}", parent_1.get_path());
        // println!("{:?}", parent_2.get_path());

        while !cycle.contains(&current) {
            cycle.push(current);
            let i = parent_1_path.iter().position(|&x| x == current).unwrap();
            availiable_pos.retain(|&x| x != i); // Keep the other values
            path[i] = current;
            current = parent_2_path[i];
        }

        let mut av_i = 0;
        for i in 0..n{
            if !cycle.contains(&parent_2_path[i]) {
                path[availiable_pos[av_i]] = parent_2_path[i];
                av_i += 1;
            }
        }
        // println!("{:?}", &path);
        let distance = Self::calculate_path_distance(&path, &self.distance_matrix);
        Chromossome::new(path, distance)
    }
    fn position_based_crossover() {
        
    }
    fn heuristic_crossover() {}
    fn genetic_edge_recombination_crossover() {}
    fn sorted_match_crossover() {}
    fn maximal_preservative_crossover() {}
    fn voting_recombination_crossover() {}
    fn partially_mapped_crossover() {}
    fn cycle_crossover_v2() {}


    fn execute_for_population(&mut self, mut sorted_population: Vec<Chromossome>, population_size: usize, initial_swap: usize) -> Vec<Chromossome> {
        let mut previous_distance = sorted_population[0].get_distance().clone();
        let mut gen_not_changed_best = 0;
        let mut gen_not_changed_best_limit = self.cities.len();
        let gen_not_changed_best_breakpoint = 200000; 
        let mut swap = initial_swap;
        // for i in 0..2 {
        while gen_not_changed_best < gen_not_changed_best_breakpoint {
            let (parent_1, parent_2) = self.select_parents(&sorted_population);
            let children = self.cycle_crossover(&parent_1, &parent_2);
            // sorted_population[population_size-1] = children.clone();
            sorted_population[population_size-1] = children.clone().mutate(&self.distance_matrix, swap);
            // for i in 2..population_size-2 {
            //     sorted_population[i] = children.clone().mutate(&self.distance_matrix, swap);
            // }
            // sorted_population[population_size-1] = sorted_population[population_size-1].clone().mutate(&self.distance_matrix, swap);
            
            sorted_population = self.sort_population_by_distance(&sorted_population);
            let best_distance = sorted_population[0].get_distance();
            if previous_distance > *best_distance {
                previous_distance = best_distance.clone();
                let mutation = sorted_population[0].get_mutation();
                self.mutations.insert(mutation.clone());
                println!("{} {} {} {}", &best_distance, self.generations, swap, mutation);
                if swap > 1 {
                    swap -= 1;
                }
                gen_not_changed_best = 0;
            } else {
                gen_not_changed_best += 1;
            }
            if gen_not_changed_best > gen_not_changed_best_limit {
                gen_not_changed_best_limit *= 2;
                if swap < self.cities.len() {
                    swap += 1;
                } else {
                    swap = 1;
                }
            }
            self.generations += 1;
        }

        sorted_population
    }

    fn execute_for_one_population_army(&mut self, first_gen: &Chromossome, initial_swap: usize) -> Chromossome {
        let mut previous_distance = first_gen.get_distance().clone();
        let mut gen_not_changed_best = 0;
        let mut gen_not_changed_best_limit = self.cities.len(); // Used to increase randomness
        let gen_not_changed_best_breakpoint = 500000; // Used to stop the algorithm
        let mut swap = initial_swap;
        let mut current_gen = first_gen.clone();

        while gen_not_changed_best < gen_not_changed_best_breakpoint {
            let new_gen = current_gen.clone().mutate(&self.distance_matrix, swap);
            let new_distance = new_gen.get_distance();
            if previous_distance > *new_distance {
                previous_distance = new_distance.clone();
                current_gen = new_gen.clone();
                let mutation = new_gen.get_mutation();
                self.mutations.insert(mutation.clone());
                println!("{} {} {} {}", &new_distance, self.generations, swap, mutation);
                if swap > 1 {
                    swap -= 1;
                }
                gen_not_changed_best = 0;
            } else {
                gen_not_changed_best += 1;
            }
            if gen_not_changed_best > gen_not_changed_best_limit {
                gen_not_changed_best_limit *= 2;
                if swap < self.cities.len() {
                    swap += 1;
                } else {
                    swap = 1;
                }
            }
            self.generations += 1;
        }
        current_gen
    }
}

// TODO: Mais mutações (greedy)
// TODO: Testar outros crossovers
impl Algorithm for Genetic {
    fn execute(&mut self) -> ExecuteResponse {
        println!("Execute Genetic");
        let start_time = Instant::now();
        let len_cities = self.cities.len();
        let population_size;
        if len_cities > 400 {
            population_size = 1;
        } else {
            population_size = 100;
        }
        self.distance_matrix = Self::create_distance_matrix(&self.cities);
        
        let population = self.create_random_population(population_size);
        let mut sorted_population = self.sort_population_by_distance(&population);

        let first_gen_best_path = sorted_population[0].get_path().clone();

        let swap = 1;

        if population_size == 1 {
            sorted_population[0] = self.execute_for_one_population_army(&sorted_population[0], swap);
        } else {
            sorted_population = self.execute_for_population(sorted_population, population_size, swap);
        }

        let metadata = format!("Population Size: {}\nGenerations: {}\nCrossover: {}\nMutations: {:?}", 
            population_size,
            self.generations,
            self.crossover,
            self.mutations
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