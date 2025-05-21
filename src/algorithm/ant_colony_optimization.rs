use std::{process::exit, time::Instant};

use rand::{Rng, rng, rngs::ThreadRng};

use super::algorithm::{Algorithm, ExecuteResponse};
use crate::models::city::City;
use std::collections::HashSet;

pub struct AntColonyOptimization {
    cities: Vec<City>,
    alpha: f64,
    beta: f64,
    vaporation_rate: f64,
    rng: ThreadRng,
    num_ants: usize,
    best_path: Vec<u16>,
    best_cost: f64,
    stall_limit: usize,
}

impl AntColonyOptimization {
    pub fn new(cities: &Vec<City>) -> Self {
        AntColonyOptimization {
            cities: cities.clone(),
            alpha: 1.0,
            beta: 1.0,
            vaporation_rate: 0.75,
            rng: rng(),
            num_ants: 10,
            best_path: vec![],
            best_cost: f64::MAX,
            stall_limit: 1000,
        }
    }

    pub fn calc_vaporation() {}

    pub fn update_pheromone(
        &mut self,
        pheromone_matrix: &mut Vec<f64>,
        paths: &Vec<(Vec<u16>, f64)>,
        q: f64,
    ) {
        let n = self.cities.len();
        // Vaporation
        for tau in pheromone_matrix.iter_mut() {
            *tau *= 1.0 - self.vaporation_rate;
        }

        for (path, cost) in paths {
            for w in path.windows(2) {
                let i = w[0] as usize;
                let j = w[1] as usize;
                let delta = q / cost;
                pheromone_matrix[i * n + j] += delta;
                pheromone_matrix[j * n + i] += delta;
            }
        }
    }

    pub fn create_pheromone_matrix(&mut self, n: usize) -> Vec<f64> {
        let mut pheromone_matrix = vec![0.0; n * n];

        for i in 0..n {
            for j in 0..n {
                if i != j {
                    pheromone_matrix[i * n + j] = 1.0; // Initialize pheromone levels
                }
            }
        }

        pheromone_matrix
    }

    pub fn chose_next_city(
        &mut self,
        current: usize,
        visited: &HashSet<u16>,
        pheromone_matrix: &Vec<f64>,
        distance_matrix: &Vec<f64>,
    ) -> usize {
        let n = self.cities.len();
        let mut probabilities = Vec::new();
        let mut sum = 0.0;

        for i in 0..n {
            if !visited.contains(&(i as u16)) {
                let tau = pheromone_matrix[current * n + i].powf(self.alpha);
                let eta = (1.0 / distance_matrix[current * n + i]).powf(self.beta);

                let probability = tau * eta;
                probabilities.push((i, probability));
                sum += probability;
            }
        }

        // rolette wheel selection
        let mut r = self.rng.random_range(0.0..1.0) * sum;
        for (j, prob) in &probabilities {
            r -= prob;
            if r <= 0.0 {
                return *j;
            }
        }

        probabilities.last().unwrap().0
    }

    pub fn run(
        &mut self,
        path: &mut Vec<u16>,
        visited: &mut HashSet<u16>,
        distance_matrix: &Vec<f64>,
        pheromone_matrix: &Vec<f64>,
    ) {
        // get random number between 0..self.cities.len
        let n = self.cities.len();
        let start_city = self.rng.random_range(0..n - 1) as u16;
        path.push(start_city);
        visited.insert(start_city);
        while path.len() < n {
            let current_city = path.last().unwrap().clone();
            let next_city = self.chose_next_city(
                current_city as usize,
                visited,
                pheromone_matrix,
                distance_matrix,
            );

            path.push(next_city as u16);
            visited.insert(next_city as u16);
        }
        // path.push(start_city);
    }

    pub fn start(&mut self) -> (Vec<u16>, f64) {
        let size = self.cities.len();
        let distance_matrix = Self::create_distance_matrix(&self.cities);
        let mut pheromone_matrix = self.create_pheromone_matrix(size);

        let stall_limit = 1000;
        let mut iterations_without_improvement = 0;
        let mut iteration = 0;
        loop {
            let mut paths: Vec<(Vec<u16>, f64)> = vec![];
            let mut improved = false;

            for _ in 0..self.num_ants {
                let mut path = vec![];
                let mut visited = HashSet::new();

                self.run(&mut path, &mut visited, &distance_matrix, &pheromone_matrix);

                // println!("Path: {:?}", path);
                let cost = Self::calculate_path_distance(&path, &distance_matrix);

                paths.push((path.clone(), cost));

                if cost < self.best_cost {
                    // println!("NEW BEST PATH (it {}): {:.4}", iteration, cost);
                    self.best_cost = cost;
                    self.best_path = path.clone();
                    improved = true;
                }
            }

            if improved {
                iterations_without_improvement = 0;
            } else {
                iterations_without_improvement += 1;
            }

            if iterations_without_improvement >= self.stall_limit {
                println!(
                    "Converged after {} iterations without improvement (best cost: {:.4})",
                    iterations_without_improvement, self.best_cost
                );
                break;
            }

            self.update_pheromone(&mut pheromone_matrix, &paths, 10.0);

            if iteration % 10 == 0 {
                println!("Iteration {}: Best Cost = {:.4}", iteration, self.best_cost);
            }
            iteration += 1;
        }
        println!(
            "Final best path: {:?} | cost = {:.4}",
            self.best_path, self.best_cost
        );

        return (self.best_path.clone(), self.best_cost);
    }
}

impl Algorithm for AntColonyOptimization {
    fn execute(&mut self) -> ExecuteResponse {
        println!("Execute AntColonyOptimization");
        let start_time = Instant::now();
        let result = self.start();
        let best_path = result.0;
        let best_cost = result.1;

        ExecuteResponse::new(
            vec![],
            best_path,
            best_cost,
            start_time.elapsed(),
            String::new(),
        )
    }
}
