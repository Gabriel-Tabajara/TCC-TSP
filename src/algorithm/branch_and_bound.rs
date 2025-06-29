use std::time::Instant;

use super::algorithm::{Algorithm, ExecuteResponse};
use crate::models::city::City;

pub struct BranchAndBound {
    cities: Vec<City>,
    size: usize,
    distance_matrix: Vec<f64>,
    best_cost: f64,
    best_path: Vec<u16>,
    call_count: usize,
    max_call_count: usize,
}

impl BranchAndBound {
    pub fn new(cities: &Vec<City>) -> Self {
        let bb = BranchAndBound {
            cities: cities.clone(),
            size: cities.len(),
            distance_matrix: Self::create_distance_matrix(cities),
            best_cost: f64::MAX,
            best_path: vec![0; cities.len() + 1],
            call_count: 0,
            max_call_count: 500_000_000,
        };

        bb
    }

    fn uptade_best_path(&mut self, path: &mut Vec<u16>) {
        for i in 0..self.size {
            self.best_path[i] = path[i];
        }
        self.best_path[self.size] = path[0];

        self.call_count = 0;
    }

    pub fn test(&mut self, path: &mut Vec<u16>, visited: &mut Vec<bool>, level: usize, cost: f64) {
        if self.call_count >= self.max_call_count {
            return;
        }
        self.call_count += 1;
        if self.call_count % 10_000_000 == 0 && self.call_count > 0 {
            println!("Call count: {}", self.call_count);
        }

        let curr_path = path.clone();

        if level == self.size {
            let total_cost = cost
                + self.distance_matrix
                    [Self::matrix_index(self.size, path[0].into(), path[level - 1].into())];
            // [Self::matrix_index(self.size, path[level - 1].into(), path[0].into())];
            if total_cost < self.best_cost {
                self.best_cost = total_cost;
                self.uptade_best_path(path);
            }
        }

        for i in 1..self.size {
            if visited[i] {
                continue;
            };

            let new_cost = cost
                + self.distance_matrix[Self::matrix_index(self.size, i, path[level - 1].into())];

            if new_cost < self.best_cost {
                visited[i] = true;
                path[level] = i as u16;
                self.test(path, visited, level + 1, new_cost);
                visited[i] = false;
            }
        }

        // println!("Distance from {} to {}: {}", i, j, distance);

        // ExecuteResponse::new(vec![], vec![], 0.0, Instant::now().elapsed(), String::new())
    }
}

impl Algorithm for BranchAndBound {
    fn execute(&mut self) -> ExecuteResponse {
        // TBD
        println!("Execute BranchAndBound");
        println!("Size: {}", self.size);

        let mut visited: Vec<bool> = vec![false; self.size];
        let mut path: Vec<u16> = vec![0; self.size + 1];

        visited[0] = true;
        let start_time = Instant::now();

        self.test(&mut path, &mut visited, 1, 0.0);

        let end = Instant::now();

        for i in 0..self.best_path.len() {
            print!("{} ", self.best_path[i])
        }
        println!("  | {}", self.best_cost);

        ExecuteResponse::new(
            vec![],
            self.best_path.clone(),
            self.best_cost,
            start_time.elapsed(),
            format!("total calls: {}", self.call_count),
        )
    }
}
