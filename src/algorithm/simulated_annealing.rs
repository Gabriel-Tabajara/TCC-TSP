use std::{collections::BinaryHeap, time::Instant};

use ordered_float::OrderedFloat;
use rand::{Rng, rng, rngs::ThreadRng, seq::SliceRandom};

use super::algorithm::{Algorithm, ExecuteResponse};
use crate::models::city::City;

#[derive(Debug, Clone)]
struct Solution {
    path: Vec<u16>,
    distance: f64,
    rng: ThreadRng,
}

impl Solution {
    fn new(path: Vec<u16>, distance: f64) -> Self {
        Solution {
            path,
            distance,
            rng: rng(),
        }
    }

    fn get_path(&self) -> &Vec<u16> {
        &self.path
    }

    fn get_distance(&self) -> &f64 {
        &self.distance
    }

    fn update_distance(mut self, path: Vec<u16>, distance_matrix: &[f64]) -> Self {
        let updated_distance = SimulatedAnnealing::calculate_path_distance(&path, distance_matrix);
        self.distance = updated_distance;
        self.path = path;
        self
    }

    fn swap(mut self, distance_matrix: &[f64]) -> Self {
        let n = &self.path.len();
        let mut path = self.path.clone();
        let first = self.rng.random_range(0..n - 1);
        let second = self.rng.random_range(0..n - 1);

        path.swap(first, second);

        self.update_distance(path, distance_matrix)
    }
}

pub struct SimulatedAnnealing {
    cities: Vec<City>,
    distance_matrix: Vec<f64>,
    rng: ThreadRng,
}

impl SimulatedAnnealing {
    pub fn new(cities: &Vec<City>) -> Self {
        SimulatedAnnealing {
            cities: cities.clone(),
            distance_matrix: vec![],
            rng: rng(),
        }
    }

    // heuristic augmented instance-based sampling strategy
    fn create_new_solution_by_heuristic_strategy(&mut self, city: u16, solution: Solution) {
        let solution_y = self.create_random_solution();
        let x_path = solution.get_path();
        let y_path = solution_y.get_path();
        let pos_city_in_x = x_path.iter().position(|&x| x == city).unwrap();
        let pos_city_in_y = y_path.iter().position(|&x| x == city).unwrap();
        let city_j = y_path[(pos_city_in_y + 1) % y_path.len()];
        if city_j == x_path[(pos_city_in_x + 1) % x_path.len()] {}
    }

    fn create_random_solution(&mut self) -> Solution {
        let mut path: Vec<u16> = (0..=self.cities.len() - 1).map(|x| x as u16).collect();
        path.shuffle(&mut self.rng);
        Solution::new(
            path.clone(),
            Self::calculate_path_distance(&path, &self.distance_matrix),
        )
    }

    fn create_solutions(&mut self, n: usize) -> Vec<Solution> {
        let mut solutions = vec![];
        while solutions.len() < n {
            solutions.push(self.create_random_solution());
        }
        solutions
    }

    fn create_temperature_list(&mut self, len: usize) -> BinaryHeap<OrderedFloat<f64>> {
        let mut current_solution = self.create_random_solution();
        let mut priority_list = vec![];

        while priority_list.len() < 2 * len {
            let new_solution = current_solution.clone().swap(&self.distance_matrix);
            let distance_new = new_solution.get_distance();
            let distance_curr = current_solution.get_distance();
            priority_list.push(OrderedFloat((distance_new - distance_curr).abs()));
            if distance_new < current_solution.get_distance() {
                current_solution = new_solution.clone()
            }
        }

        priority_list.sort_unstable();
        priority_list.drain(0..len / 2);
        priority_list.truncate(len);
        BinaryHeap::from(priority_list)
    }

    fn create_temperature_lists_matrix(
        &mut self,
        size: usize,
        temp_list_len: usize,
    ) -> Vec<BinaryHeap<OrderedFloat<f64>>> {
        let mut matrix: Vec<BinaryHeap<OrderedFloat<f64>>> = Vec::with_capacity(size);

        for _ in 0..size {
            let temperature_list = self.create_temperature_list(temp_list_len);
            matrix.push(temperature_list.into_iter().take(temp_list_len).collect());
        }

        matrix
    }

    fn create_mcl_list(&mut self, pos: f64, mcl: usize, generations: usize) -> Vec<usize> {
        let best_gen = ((generations as f64) * pos).round();
        let mut a_mcl = Vec::with_capacity(generations);

        for i in 0..generations {
            let ratio;
            if i as f64 <= best_gen {
                ratio = (((i as f64) / best_gen) * mcl as f64).round();
            } else {
                ratio = ((generations as f64 - 1.0 - i as f64)
                    / (generations as f64 - 1.0 - best_gen)
                    * mcl as f64)
                    .round();
            }
            let i_mcl = mcl / 2 + ratio as usize;
            a_mcl.push(i_mcl);
        }

        a_mcl
    }

    fn find_best_solution(&self, population: &[Solution]) -> Solution {
        population
            .iter()
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
            .unwrap()
            .clone()
    }
}

// Falta fazer com greedy no inicio que nem no paper, e parar de gerar cidades random
impl Algorithm for SimulatedAnnealing {
    fn execute(&mut self) -> ExecuteResponse {
        println!("Execute SimulatedAnnealing");
        let start_time = Instant::now();
        let cities_len = self.cities.len();
        let temp_list_len = 150;
        //p
        let population_size;
        if cities_len < 1000 {
            population_size = 50;
        } else {
            population_size = 20;
        }
        //g
        let generations = 10;
        //m
        let markov_chain_len = cities_len;
        let pos = 0.375;
        self.distance_matrix = Self::create_distance_matrix(&self.cities);

        let a_sol = self.create_solutions(population_size);
        let mut tempreture_matrix =
            self.create_temperature_lists_matrix(population_size, temp_list_len);
        let mut a_city = vec![0; population_size];
        let a_mcl = self.create_mcl_list(pos, markov_chain_len, generations);
        let best = self.find_best_solution(&a_sol);
        for g in 0..generations {
            for i in 0..population_size {
                let current_solution = &a_sol[i];
                let temperature = tempreture_matrix[g].pop().unwrap();
                let (mut k, mut c, mut s) = (0, 0, 0);
                while k < a_mcl[g] {
                    a_city[i] = (a_city[i] + 1) % cities_len;
                    k += 1;
                }
            }
        }
        ExecuteResponse::new(vec![], vec![], 0.0, start_time.elapsed(), String::new())
    }
}
