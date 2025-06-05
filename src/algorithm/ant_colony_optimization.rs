use rand::{Rng, rng, rngs::ThreadRng};
use std::cmp;
use std::{process::exit, time::Instant};

use super::algorithm::{Algorithm, ExecuteResponse};
use crate::models::city::City;
use std::collections::HashSet;

pub struct AntColonyOptimization {
    cities: Vec<City>,
    alpha: f64,           // importance of pheromone
    beta: f64,            // importance of heuristic
    vaporation_rate: f64, // rate of pheromone evaporation
    rho0: f64,            // initial pheromone vaporation rate
    gamma: f64,           // rate of pheromone vaporation reduction
    omega: f64,           // % of iterations needed to start reducing pheromone vaporation rate
    stall_limit: usize,   // max number of iterations before stopping
    s: usize,             // number of iterations without improvement
    s_threshold: usize,   // nº of consecutive iterations without reducing pheromone vaporation rate
    rng: ThreadRng,       // random number generator
    num_ants: usize,      // number of ants to simulate
    best_path: Vec<u16>,  // best path found
    best_cost: f64,       // cost of the best path found
    candidate_lists: Vec<Vec<usize>>,
}

impl AntColonyOptimization {
    pub fn new(cities: &Vec<City>) -> Self {
        let min_ants = 50;
        let max_ants;
        if cities.len() >= 400 {
            max_ants = 50;
        } else if cities.len() >= 200 {
            max_ants = 50;
        } else {
            max_ants = 50;
        }
        let ants = cmp::max(min_ants, cmp::min(cities.len(), max_ants));
        print!(
            "AntColonyOptimization::new() called with {} cities, using {} ants",
            cities.len(),
            ants
        );
        AntColonyOptimization {
            cities: cities.clone(),
            alpha: 1.0,
            beta: 1.0,
            vaporation_rate: 0.75,
            rho0: 0.3,
            gamma: 0.8,
            omega: 0.7,
            stall_limit: 200,
            s_threshold: 30,
            s: 0,
            rng: rng(),
            num_ants: ants,
            best_path: vec![],
            best_cost: f64::MAX,
            candidate_lists: Self::build_candidate_lists(cities),
        }
    }
    pub fn update_rho(&mut self, iteration: usize) {
        if iteration < (self.omega * self.stall_limit as f64) as usize {
            self.vaporation_rate = self.rho0;
        } else if self.s > self.s_threshold {
            self.vaporation_rate *= self.gamma;
            if self.vaporation_rate < 0.01 {
                self.vaporation_rate = 0.01;
            }
        }
    }

    // Atualização de feromônio com ASrank (formigas ordenadas por qualidade de solução)
    pub fn update_pheromone(
        &mut self,
        pheromone_matrix: &mut Vec<f64>,
        distance_matrix: &Vec<f64>,
        paths: &Vec<(Vec<u16>, f64)>,
        q: f64,
    ) {
        let n = self.cities.len();
        // global pheromone evaporation
        for tau in pheromone_matrix.iter_mut() {
            *tau *= 1.0 - self.vaporation_rate;
        }

        let mut ranked_paths = paths.clone();
        ranked_paths.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Top lambda*m ants updates pheromone (ASrank)
        let lambda = (self.num_ants as f64 * 0.2).ceil() as usize;

        // if self.s % 10 == 0 {
        self.optimize_best_paths(&mut ranked_paths, distance_matrix, lambda, n);
        // }

        for (rank, (path, cost)) in ranked_paths.iter().take(lambda).enumerate() {
            let weight = (lambda - rank) as f64; // rank 0 = mais peso
            for w in path.windows(2) {
                let i = w[0] as usize;
                let j = w[1] as usize;
                let delta = weight * q / cost;
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

    pub fn use_candidate_lists(&self, current: usize) -> &Vec<usize> {
        &self.candidate_lists[current]
    }

    pub fn use_all_cities(&self) -> Vec<usize> {
        (0..self.cities.len()).collect()
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

        // // use candidate lists
        let candidates = &self.candidate_lists[current];
        let mut candidates: Vec<usize> = candidates
            .iter()
            .copied()
            .filter(|i| !visited.contains(&(*i as u16)))
            .collect();
        if candidates.is_empty() {
            candidates = (0..n)
                .filter(|i| !visited.contains(&(*i as u16)) && *i != current)
                .collect();
        }

        for i in candidates {
            let tau = pheromone_matrix[current * n + i].powf(self.alpha);
            let eta = (1.0 / distance_matrix[current * n + i]).powf(self.beta);
            let probability = tau * eta;
            probabilities.push((i, probability));
            sum += probability;
        }

        // use all cities
        // for i in 0..n {
        //     if !visited.contains(&(i as u16)) {
        //         let tau = pheromone_matrix[current * n + i].powf(self.alpha);
        //         let eta = (1.0 / distance_matrix[current * n + i]).powf(self.beta);
        //         let probability = tau * eta;
        //         probabilities.push((i, probability));
        //         sum += probability;
        //     }
        // }

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

    fn compute_entropy(&self, pheromone_matrix: &Vec<f64>) -> f64 {
        let total: f64 = pheromone_matrix.iter().sum();
        pheromone_matrix
            .iter()
            .filter(|&&p| p > 0.0)
            .map(|&p| {
                let prob = p / total;
                -prob * prob.log2()
            })
            .sum()
    }

    fn update_beta_by_entropy(&mut self, pheromone_matrix: &Vec<f64>) {
        let entropy = self.compute_entropy(pheromone_matrix);
        let max_entropy = (self.cities.len() * (self.cities.len() - 1) / 2) as f64;
        let e_prime = 1.0 - (entropy / max_entropy);

        self.beta = if e_prime < 0.3 {
            5.0
        } else if e_prime < 0.5 {
            4.0
        } else if e_prime < 0.7 {
            3.0
        } else {
            2.0
        };
    }

    pub fn run(
        &mut self,
        path: &mut Vec<u16>,
        visited: &mut HashSet<u16>,
        distance_matrix: &Vec<f64>,
        pheromone_matrix: &Vec<f64>,
    ) {
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

    fn update_alpha_beta(&mut self, iter: usize, max_iter: usize) {
        let r1: f64 = self.rng.random();
        let r2: f64 = self.rng.random();
        let t = iter as f64;
        let t_max = max_iter as f64;

        // Fixed values for sine and cosine functions
        let a = 1.5;
        let b = 2.5;

        // Amplitude for sine and cosine functions
        let alpha_amplitude = 1.0;
        let beta_amplitude = 1.5;

        self.alpha = alpha_amplitude * (r1 * t * std::f64::consts::PI / (2.0 * t_max)).cos() + a;
        self.beta = beta_amplitude * (r2 * t * std::f64::consts::PI / (2.0 * t_max)).sin() + b;
    }

    fn two_opt(&mut self, path: &mut Vec<u16>, distance_matrix: &Vec<f64>, n: usize) -> f64 {
        let mut improved = true;
        let mut total_cost = Self::calculate_path_distance(&path, &distance_matrix);

        while improved {
            improved = false;
            for i in 1..path.len() - 2 {
                for j in i + 1..path.len() - 1 {
                    let a = path[i - 1] as usize;
                    let b = path[i] as usize;
                    let c = path[j] as usize;
                    let d = path[j + 1] as usize;

                    let current = distance_matrix[a * n + b] + distance_matrix[c * n + d];
                    let new = distance_matrix[a * n + c] + distance_matrix[b * n + d];

                    if new < current {
                        path[i..=j].reverse();
                        total_cost = total_cost - current + new;
                        improved = true;
                    }
                }
            }
        }

        total_cost
    }

    pub fn optimize_best_paths(
        &mut self,
        paths: &mut Vec<(Vec<u16>, f64)>,
        distance_matrix: &Vec<f64>,
        top_k: usize,
        n: usize,
    ) {
        paths.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        for i in 0..top_k.min(paths.len()) {
            let (ref mut path, ref mut cost) = paths[i];
            *cost = self.two_opt(path, distance_matrix, n);
        }
    }

    fn build_candidate_lists(cities: &Vec<City>) -> Vec<Vec<usize>> {
        let n: usize = cities.len();
        let mut lists = vec![vec![]; n];

        for i in 0..n {
            let mut distances: Vec<(usize, f64)> = (0..n)
                .filter(|&j| i != j)
                .map(|j| {
                    (
                        j,
                        Self::calculate_distance_between_cities(&cities[i], &cities[j]),
                    )
                })
                .collect();

            distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            let list_size = (n as f64 / 4.0).ceil() as usize;
            lists[i] = distances.iter().take(list_size).map(|&(j, _)| j).collect();
        }

        lists
    }

    pub fn start(&mut self) -> (Vec<u16>, f64) {
        let size = self.cities.len();
        let distance_matrix = Self::create_distance_matrix(&self.cities);
        let mut pheromone_matrix = self.create_pheromone_matrix(size);

        let mut iterations_without_improvement = 0;
        let mut iteration = 0;
        loop {
            let mut paths: Vec<(Vec<u16>, f64)> = vec![];
            let mut improved = false;

            for _ in 0..self.num_ants {
                let mut path = vec![];
                let mut visited = HashSet::new();

                self.run(&mut path, &mut visited, &distance_matrix, &pheromone_matrix);
                let cost = Self::calculate_path_distance(&path, &distance_matrix);
                paths.push((path.clone(), cost));

                if cost < self.best_cost {
                    self.best_cost = cost;
                    self.best_path = path.clone();
                    improved = true;
                }
            }

            // self.update_alpha_beta(iteration, self.stall_limit); // AACO-LST
            self.update_beta_by_entropy(&pheromone_matrix);

            if improved {
                iterations_without_improvement = 0;
                self.s = 0;
            } else {
                iterations_without_improvement += 1;
                self.s += 1;
            }

            if iterations_without_improvement >= self.stall_limit {
                // println!(
                //     "Converged after {} iterations without improvement (best cost: {:.4})",
                //     iterations_without_improvement, self.best_cost
                // );
                break;
            }

            self.update_rho(iteration);
            self.update_pheromone(&mut pheromone_matrix, &distance_matrix, &mut paths, 10.0);

            if iteration % 5 == 0 {
                println!("Iteration {}: Best Cost = {:.4}", iteration, self.best_cost);
            }
            iteration += 1;
        }

        // println!(
        //     "Final best path: {:?} | cost = {:.4}",
        //     self.best_path, self.best_cost
        // );

        (self.best_path.clone(), self.best_cost)
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
