use std::{collections::BinaryHeap, time::Instant};

use kiddo::KdTree;
use ordered_float::OrderedFloat;
use rand::{
    Rng, rng,
    rngs::ThreadRng,
    seq::{IndexedRandom, SliceRandom},
};

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

    fn update_distance(mut self, distance_matrix: &[f64]) -> Self {
        let updated_distance =
            SimulatedAnnealing::calculate_path_distance(&self.path, distance_matrix);
        self.distance = updated_distance;
        self
    }

    fn swap(mut self, distance_matrix: &[f64]) -> Self {
        let n = &self.path.len();
        let first = self.rng.random_range(0..n - 1);
        let second = self.rng.random_range(0..n - 1);

        self.path.swap(first, second);

        self.update_distance(distance_matrix)
    }

    fn swap_sampling(mut self, distance_matrix: &[f64], city_i: u16, city_j: u16) -> Self {
        let city_i_index = self.path.iter().position(|&x| x == city_i).unwrap();
        let city_j_index = self.path.iter().position(|&x| x == city_j).unwrap();

        self.path.swap(city_i_index, city_j_index);

        self.update_distance(distance_matrix)
    }

    fn block_insert_sampling(mut self, distance_matrix: &[f64], city_i: u16, city_j: u16) -> Self {
        let city_i_index = self.path.iter().position(|&x| x == city_i).unwrap();
        let city_j_index = self.path.iter().position(|&x| x == city_j).unwrap();
        let path_len = self.path.len();

        let random = self.rng.random_range(1..10);
        let block_size = random
            .min(((city_i_index as isize - city_j_index as isize - 1) as isize).abs() as usize);

        if block_size == 0 {
            return self;
        }

        if city_j_index + block_size < path_len {
            let displaced_part: Vec<u16> = self
                .path
                .drain(city_j_index..city_j_index + block_size)
                .collect();
            if city_j_index < city_i_index {
                self.path.splice(
                    city_i_index - block_size..city_i_index - block_size,
                    displaced_part,
                );
            } else {
                self.path.splice(city_i_index..city_i_index, displaced_part);
            }
        } else {
            let mut block = vec![];
            for i in 0..block_size {
                let pos = (city_j_index + i) % path_len;
                if self.path[pos] == city_i {
                    break;
                }
                block.push(self.path[pos]);
            }
            self.path.retain(|x| !block.contains(x));
            let new_city_index = self.path.iter().position(|&x| x == city_i).unwrap();
            self.path.splice(new_city_index..new_city_index, block);
        }

        self.update_distance(distance_matrix)
    }

    fn inverse_sampling(mut self, distance_matrix: &[f64], city_i: u16, city_j: u16) -> Self {
        let city_i_index = self.path.iter().position(|&x| x == city_i).unwrap();
        let city_j_index = self.path.iter().position(|&x| x == city_j).unwrap();

        if city_i_index < city_j_index {
            self.path[city_i_index + 1..city_j_index + 1].reverse();
        } else {
            self.path[city_j_index + 1..city_i_index + 1].reverse();
        }

        self.update_distance(distance_matrix)
    }
}

pub struct SimulatedAnnealing {
    cities: Vec<City>,
    distance_matrix: Vec<f64>,
    kd_tree: KdTree<f32, 2>,
    rng: ThreadRng,
}

impl SimulatedAnnealing {
    pub fn new(cities: &Vec<City>) -> Self {
        SimulatedAnnealing {
            cities: cities.clone(),
            distance_matrix: vec![],
            kd_tree: SimulatedAnnealing::create_kd_tree(cities.clone()),
            rng: rng(),
        }
    }

    // heuristic augmented instance-based sampling strategy
    fn create_new_solution_by_heuristic_strategy(
        &mut self,
        city: u16,
        solution: &Solution,
        a_sol: &Vec<Solution>,
    ) -> Solution {
        let random = self.rng.random_range(0..a_sol.len() - 1);
        let solution_y = &a_sol[random];
        let x_path = solution.get_path();
        let y_path = solution_y.get_path();
        let pos_city_in_x = x_path.iter().position(|&x| x == city).unwrap();
        let pos_city_in_y = y_path.iter().position(|&x| x == city).unwrap();
        let mut city_j = y_path[(pos_city_in_y + 1) % y_path.len()];
        if city_j == x_path[(pos_city_in_x + 1) % x_path.len()] {
            let leading_index;
            if pos_city_in_y == 0 {
                leading_index = y_path.len() - 1;
            } else {
                leading_index = pos_city_in_y - 1
            }
            city_j = y_path[(leading_index) % y_path.len()];
        }
        let x_1 = solution
            .clone()
            .inverse_sampling(&self.distance_matrix, city, city_j);
        let x_2 = solution
            .clone()
            .swap_sampling(&self.distance_matrix, city, city_j);
        let x_3 = solution
            .clone()
            .block_insert_sampling(&self.distance_matrix, city, city_j);

        self.find_best_solution(&[x_1, x_2, x_3])
    }

    fn create_random_solution(&mut self) -> Solution {
        let mut path: Vec<u16> = (0..=self.cities.len() - 1).map(|x| x as u16).collect();
        path.shuffle(&mut self.rng);
        Solution::new(
            path.clone(),
            Self::calculate_path_distance(&path, &self.distance_matrix),
        )
    }

    fn create_greedy_solution(&mut self, greedy_range: usize) -> Solution {
        let cities_len = self.cities.len();

        let greedy_n: usize = self.rng.random_range(1..greedy_range + 1);
        let mut current_city = self.rng.random_range(0..cities_len - 1);

        let mut path = vec![current_city];
        let mut visited_cities = vec![&self.cities[current_city]];
        while path.len() < cities_len {
            let near_neighbours = SimulatedAnnealing::find_best_n_neighbours_kd_tree(
                &self.kd_tree,
                &self.cities[current_city],
                greedy_n,
                &visited_cities,
            );

            let chosen = near_neighbours.choose(&mut self.rng).unwrap();

            current_city = chosen.clone();
            path.push(current_city);
            visited_cities.push(&self.cities[current_city]);
        }

        let path_u16 = path.iter().map(|&x| x as u16).collect::<Vec<u16>>().clone();

        Solution::new(
            path_u16.clone(),
            Self::calculate_path_distance(&path_u16, &self.distance_matrix),
        )
    }

    fn create_random_solutions(&mut self, n: usize) -> Vec<Solution> {
        let mut solutions = vec![];
        while solutions.len() < n {
            solutions.push(self.create_random_solution());
        }
        solutions
    }

    fn create_greedy_solutions(&mut self, n: usize, greedy_range: usize) -> Vec<Solution> {
        let mut solutions = vec![];
        while solutions.len() < n {
            solutions.push(self.create_greedy_solution(greedy_range));
        }
        solutions
    }

    fn create_temperature_list(
        &mut self,
        len: usize,
        greedy_range: usize,
    ) -> BinaryHeap<OrderedFloat<f64>> {
        let mut current_solution = self.create_greedy_solution(greedy_range);
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
        greedy_range: usize,
    ) -> Vec<BinaryHeap<OrderedFloat<f64>>> {
        let mut matrix: Vec<BinaryHeap<OrderedFloat<f64>>> = Vec::with_capacity(size);

        for _ in 0..size {
            let temperature_list = self.create_temperature_list(temp_list_len, greedy_range);
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

impl Algorithm for SimulatedAnnealing {
    fn execute(&mut self) -> ExecuteResponse {
        println!("Execute SimulatedAnnealing");
        let start_time = Instant::now();
        let cities_len = self.cities.len();
        let greedy_range = cities_len.max(20);
        let temp_list_len = 150;
        //p
        let population_size;
        if cities_len < 1000 {
            population_size = 50;
        } else {
            population_size = 20;
        }
        //g
        let generations = 1000;
        //m
        let markov_chain_len = cities_len;
        let pos = 0.375;
        self.distance_matrix = Self::create_distance_matrix(&self.cities);

        let mut a_sol = self.create_random_solutions(population_size);
        println!("Finalizou asol");
        let mut tempreture_matrix =
            self.create_temperature_lists_matrix(population_size, temp_list_len, greedy_range);
        println!("Finalizou tempreture_matrix");
        let mut a_city: Vec<u16> = vec![0; population_size];
        let a_mcl = self.create_mcl_list(pos, markov_chain_len, generations);
        let mut best = self.find_best_solution(&a_sol);
        let initial_best = best.clone();
        println!("Finalizou setup");
        for g in 0..generations {
            for i in 0..population_size {
                let temperature = tempreture_matrix[i].peek().unwrap();
                // println!("temp {}", temperature);
                let (mut k, mut c, mut s) = (0, 0, 0.0);
                while k < a_mcl[g] {
                    a_city[i] = (a_city[i] + 1) % (cities_len as u16);
                    let solution_y = self
                        .create_new_solution_by_heuristic_strategy(a_city[i], &a_sol[i], &a_sol);

                    let current_distance = a_sol[i].get_distance();
                    let y_distance = solution_y.get_distance();
                    let distance_diff = y_distance - current_distance;
                    let p;
                    if y_distance < current_distance {
                        p = 1.0;
                    } else {
                        p = (-distance_diff / temperature.0).exp()
                    }
                    let random = self.rng.random_range(0.0..1.0);
                    if random < p {
                        if distance_diff > 0.0 {
                            s += -distance_diff / random.ln();
                            c += 1;
                        } else if current_distance < best.get_distance() {
                            best = a_sol[i].clone();
                            println!("{} {}", &best.get_distance(), g);
                        }
                        a_sol[i] = solution_y.clone();
                        // println!("k {} p {} g {}", k, i, g);
                    }
                    k += 1;
                }
                if c > 0 {
                    tempreture_matrix[i].pop();
                    tempreture_matrix[i].push(OrderedFloat(s / c as f64));
                    if (s / c as f64) < 0.0 {
                        println!("s {} c {}", s, c);
                    }
                }
            }
            if g % 50 == 0 {
                println!("gen {}", g);
            }
        }
        ExecuteResponse::new(
            initial_best.get_path().clone(),
            best.get_path().clone(),
            best.get_distance().clone(),
            start_time.elapsed(),
            String::new(),
        )
    }
}
