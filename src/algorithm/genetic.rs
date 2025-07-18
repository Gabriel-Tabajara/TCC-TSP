use std::{
    collections::{HashMap, HashSet},
    time::Instant,
    usize,
};

use super::algorithm::{Algorithm, ExecuteResponse};
use crate::models::city::City;
use kiddo::KdTree;
use plotters::prelude::LogScalable;
use rand::{
    Rng,
    distr::{Distribution, weighted::WeightedIndex},
    rng,
    rngs::ThreadRng,
    seq::{IndexedRandom, SliceRandom},
};

#[derive(Debug, Clone)]
struct Chromossome {
    path: Vec<u16>,
    distance: f64,
    mutation: String,
    rng: ThreadRng,
}

impl PartialEq for Chromossome {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
            && self.distance == other.distance
            && self.mutation == other.mutation
    }
}

impl Chromossome {
    fn new(path: Vec<u16>, distance: f64) -> Self {
        Chromossome {
            path,
            distance,
            mutation: "".to_string(),
            rng: rng(),
        }
    }

    fn get_path(&self) -> &Vec<u16> {
        &self.path
    }

    fn get_distance(&self) -> &f64 {
        &self.distance
    }

    fn get_mutation(&self) -> &String {
        &self.mutation
    }

    fn update_distance(mut self, path: Vec<u16>, distance_matrix: &[f64], mutation: &str) -> Self {
        let updated_distance = Genetic::calculate_path_distance(&path, distance_matrix);
        if &updated_distance < &self.distance {
            self.distance = updated_distance;
            self.path = path;
            self.mutation = mutation.to_string();
        }
        self
    }

    fn mutate(mut self, distance_matrix: &[f64], swaps: usize) -> Self {
        let prob = self.rng.random_range(0.0..1.0);
        if prob <= 0.18 {
            self.swap_mutation(distance_matrix, swaps)
        } else if prob <= 0.36 {
            self.displacement_mutation(distance_matrix)
        } else if prob <= 0.54 {
            self.insertion_mutation(distance_matrix)
        } else if prob <= 0.72 {
            self.simple_inversion_mutation(distance_matrix)
        } else if prob <= 0.96 {
            self.inversion_mutation(distance_matrix)
        } else if prob <= 0.98 {
            self.greedy_sub_tour_mutation(distance_matrix)
        } else {
            self.greedy_insertion_mutation(distance_matrix)
        }
    }

    fn swap_mutation(mut self, distance_matrix: &[f64], swaps: usize) -> Self {
        let n = &self.path.len();
        let mut path = self.path.clone();
        for i in 0..swaps {
            let first = self.rng.random_range(0..n - 1);
            let second = self.rng.random_range(0..n - 1);

            path.swap(first, second);
        }

        self.update_distance(path, distance_matrix, "swap_mutation")
    }

    fn displacement_mutation(mut self, distance_matrix: &[f64]) -> Self {
        let n = &self.path.len();
        let mut path = self.path.clone();
        let shift_size = self.rng.random_range(2..n - 1);
        let distance2 = self.rng.random_range(0..n - 1);

        let shift_position = self.rng.random_range(0..n - shift_size - 1);
        let displaced_part: Vec<u16> = path
            .drain(shift_position..shift_position + shift_size)
            .collect();

        let new_position = (shift_position + distance2) % (n - shift_size);
        path.splice(new_position..new_position, displaced_part);

        self.update_distance(path, distance_matrix, "displacement_mutation")
    }

    fn insertion_mutation(mut self, distance_matrix: &[f64]) -> Self {
        let n = &self.path.len();
        let mut path = self.path.clone();
        let old_pos = self.rng.random_range(1..n - 1);
        let mut new_pos = old_pos;
        while new_pos == old_pos {
            new_pos = self.rng.random_range(1..n - 1);
        }
        let city = path.remove(old_pos);
        path.insert(new_pos, city);

        self.update_distance(path, distance_matrix, "insertion_mutation")
    }

    fn simple_inversion_mutation(mut self, distance_matrix: &[f64]) -> Self {
        let n = &self.path.len();
        let mut path = self.path.clone();

        let start = self.rng.random_range(0..n - 3);
        let size = self.rng.random_range(2..n - start);

        path[start..start + size].reverse();

        self.update_distance(path, distance_matrix, "simple_inversion_mutation")
    }

    fn inversion_mutation(mut self, distance_matrix: &[f64]) -> Self {
        let n = &self.path.len();
        let mut path = self.path.clone();
        let shift_size = self.rng.random_range(2..n - 1);
        let distance2 = self.rng.random_range(0..n - 1);

        let shift_position = self.rng.random_range(0..n - shift_size - 1);
        let displaced_reversed_part: Vec<u16> = path
            .drain(shift_position..shift_position + shift_size)
            .rev()
            .collect();

        let new_position = (shift_position + distance2) % (n - shift_size);
        path.splice(new_position..new_position, displaced_reversed_part);

        self.update_distance(path, distance_matrix, "inversion_mutation")
    }

    fn greedy_sub_tour_mutation(mut self, distance_matrix: &[f64]) -> Self {
        let n = &self.path.len();
        let mut path = self.path.clone();

        let min_sub_tour = 2;
        let max_sub_tour = (*n as f64).sqrt() as usize;

        let start = self.rng.random_range(0..n - max_sub_tour);
        let size = self
            .rng
            .random_range(min_sub_tour..max_sub_tour.max(min_sub_tour));

        let sub_tour: Vec<u16> = path.drain(start..start + size).collect();

        let sub_tour_usize: Vec<usize> = sub_tour.iter().map(|&x| x as usize).collect();
        let first_best = Genetic::find_best_neighbour(
            distance_matrix,
            sub_tour[0] as usize,
            *n,
            &sub_tour_usize,
        );
        let second_best = Genetic::find_best_neighbour(
            distance_matrix,
            sub_tour[sub_tour.len() - 1] as usize,
            *n,
            &sub_tour_usize,
        );

        let first_i = path.iter().position(|&x| x == first_best as u16).unwrap();
        let second_i = path.iter().position(|&x| x == second_best as u16).unwrap();

        let mut first_path: Vec<u16> = path.clone();
        first_path.splice(first_i + 1..first_i + 1, sub_tour.clone());
        if first_i + 1 != second_i {
            let mut second_path: Vec<u16> = path.clone();
            second_path.splice(second_i..second_i, sub_tour.clone());

            let first_path_distance =
                Genetic::calculate_path_distance(&first_path, distance_matrix);
            let second_path_distance =
                Genetic::calculate_path_distance(&second_path, distance_matrix);

            if second_path_distance < first_path_distance {
                path = second_path;
            } else {
                path = first_path
            }
        } else {
            path = first_path
        }

        self.update_distance(path, distance_matrix, "greedy_sub_tour_mutation")
    }

    fn greedy_insertion_mutation(mut self, distance_matrix: &[f64]) -> Self {
        let n = self.path.len();
        let mut path = self.path.clone();

        let min_neighbour = 5;
        let max_neighbour = (n as f64).sqrt() as usize;
        let size = min_neighbour.max(max_neighbour);

        let city = self.rng.random_range(0..n - 1);
        path.retain(|&x| x != city as u16);

        let near_neighbours = Genetic::find_n_best_neighbours(distance_matrix, city, n, size);

        let chosen = near_neighbours.choose(&mut self.rng).unwrap();
        let chosen_i = path.iter().position(|&x| x == *chosen as u16).unwrap();

        if self.rng.random_bool(0.5) {
            path.insert(chosen_i + 1, city as u16);
        } else {
            path.insert(chosen_i, city as u16);
        }

        self.update_distance(path, distance_matrix, "greedy_insertion_mutation")
    }
}

pub struct Genetic {
    distance_matrix: Vec<f64>,
    kd_tree: KdTree<f32, 2>,
    cities: Vec<City>,
    crossover: String,
    mutations: HashSet<String>,
    generations: u32,
    rng: ThreadRng,
}

impl Genetic {
    pub fn new(cities: &Vec<City>) -> Self {
        Genetic {
            distance_matrix: vec![],
            cities: cities.clone(),
            kd_tree: Genetic::create_kd_tree(cities.clone()),
            crossover: String::new(),
            mutations: HashSet::new(),
            generations: 0,
            rng: rng(),
        }
    }

    fn create_random_population(&mut self, n: usize) -> Vec<Chromossome> {
        let mut population: Vec<Chromossome> = Vec::with_capacity(n);
        let mut current_path: Vec<u16> = (0..=self.cities.len() - 1).map(|x| x as u16).collect();

        while population.len() < n {
            current_path.shuffle(&mut self.rng);
            population.push(Chromossome::new(
                current_path.clone(),
                Self::calculate_path_distance(&current_path, &self.distance_matrix),
            ));
        }

        population
    }

    fn create_greedy_population(&mut self, n: usize, greedy_range: usize) -> Vec<Chromossome> {
        let mut population: Vec<Chromossome> = Vec::with_capacity(n);
        let cities_len = self.cities.len();

        while population.len() < n {
            let greedy_n: usize = self.rng.random_range(1..greedy_range + 1);
            let mut current_city = self.rng.random_range(0..cities_len - 1);

            let mut path = vec![current_city];
            let mut visited_cities = vec![&self.cities[current_city]];
            while path.len() < cities_len {
                let near_neighbours = Genetic::find_best_n_neighbours_kd_tree(
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

            population.push(Chromossome::new(
                path_u16.clone(),
                Self::calculate_path_distance(&path_u16, &self.distance_matrix),
            ));
            println!("Chromossome {} created", population.len());
        }

        population
    }

    fn get_best_chromossome(&self, population: &[Chromossome]) -> Chromossome {
        population
            .iter()
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
            .unwrap()
            .clone()
    }

    fn get_worst_chromossome(&self, population: &[Chromossome]) -> Chromossome {
        population
            .iter()
            .max_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
            .unwrap()
            .clone()
    }

    fn select_parents(&mut self, population: &[Chromossome]) -> (Chromossome, Chromossome) {
        let n = population.len();
        let parent_1 = self.rng.random_range(0..n - 1);
        let mut parent_2 = parent_1;
        while parent_1 == parent_2 {
            parent_2 = self.rng.random_range(0..n - 1);
        }
        (population[parent_1].clone(), population[parent_2].clone())
    }

    fn order_crossover(&mut self, parent_1: &Chromossome, parent_2: &Chromossome) -> Chromossome {
        self.crossover = "order_crossover".to_string();

        let n = parent_1.get_path().len();

        let mut start = self.rng.random_range(0..n - 2);
        let mut end = self.rng.random_range(start + 1..n - 1);

        let mut path = vec![u16::MAX; n];
        path[start..end].copy_from_slice(&parent_1.get_path()[start..end]);
        for &city in parent_2.get_path() {
            if !path.contains(&city) {
                if end <= n - 1 {
                    path[end] = city;
                    end += 1;
                } else {
                    path[start - 1] = city;
                    start -= 1;
                }
            }
        }

        let distance = Self::calculate_path_distance(&path, &self.distance_matrix);
        Chromossome::new(path, distance)
    }

    fn order_based_crossover(
        &mut self,
        parent_1: &Chromossome,
        parent_2: &Chromossome,
    ) -> Chromossome {
        self.crossover = "order_based_crossover".to_string();

        let n = parent_1.get_path().len();

        let mut start = self.rng.random_range(0..n - 2);
        let end = self.rng.random_range(start + 1..n - 1);

        let mut path = parent_1.get_path().clone();
        let removed_cities = path[start..end].to_vec();

        for &city in parent_2.get_path() {
            if removed_cities.contains(&city) {
                path[start] = city;
                start += 1;
                if start >= end {
                    break;
                }
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

        while !cycle.contains(&current) {
            cycle.push(current);
            let i = parent_1_path.iter().position(|&x| x == current).unwrap();
            availiable_pos.retain(|&x| x != i);
            path[i] = current;
            current = parent_2_path[i];
        }

        let mut av_i = 0;
        for i in 0..n {
            if !cycle.contains(&parent_2_path[i]) {
                path[availiable_pos[av_i]] = parent_2_path[i];
                av_i += 1;
            }
        }

        let distance = Self::calculate_path_distance(&path, &self.distance_matrix);
        Chromossome::new(path, distance)
    }

    fn position_based_crossover(
        &mut self,
        parent_1: &Chromossome,
        parent_2: &Chromossome,
    ) -> Chromossome {
        self.crossover = "position_based_crossover".to_string();
        let parent_2_path = parent_2.get_path();

        let n = parent_2_path.len();
        let mut path = parent_1.get_path().clone();

        let mut swap_size = self.rng.random_range(1..path.len() / 5);
        let mut swaped_positions = vec![false; n];

        while swap_size > 0 {
            let i = self.rng.random_range(0..n - 1);
            if !swaped_positions[i] {
                let to_swap = path[i];
                let j = path.iter().position(|&x| x == parent_2_path[i]).unwrap();
                path[i] = parent_2_path[i];
                path[j] = to_swap;
                swaped_positions[i] = true;
                swap_size -= 1;
            }
        }

        let distance = Self::calculate_path_distance(&path, &self.distance_matrix);
        Chromossome::new(path, distance)
    }

    fn heuristic_crossover(
        &mut self,
        parent_1: &Chromossome,
        parent_2: &Chromossome,
    ) -> Chromossome {
        self.crossover = "heuristic_crossover".to_string();
        let parent_1_path = parent_1.get_path();
        let parent_2_path = parent_2.get_path();

        let n = parent_1_path.len();
        let mut path = vec![];
        let mut unvisited = parent_1_path.clone();

        path.push(unvisited[self.rng.random_range(0..unvisited.len() - 1)]);
        let mut current = path[0];
        unvisited.retain(|&x| x != current);

        while path.len() < n {
            let mut edges = vec![];
            let j = parent_1_path.iter().position(|&x| x == current).unwrap();
            let k = parent_2_path.iter().position(|&x| x == current).unwrap();
            for (parent, i) in [(parent_1_path, j), (parent_2_path, k)] {
                if i > 0 && !path.contains(&parent[i - 1]) {
                    edges.push((
                        parent[i - 1],
                        Self::calculate_distance_between_cities_ids(
                            current as usize,
                            parent[i - 1] as usize,
                            &self.cities,
                        ),
                    ));
                }

                if i + 1 < parent.len() && !path.contains(&parent[i + 1]) {
                    edges.push((
                        parent[i + 1],
                        Self::calculate_distance_between_cities_ids(
                            current as usize,
                            parent[i + 1] as usize,
                            &self.cities,
                        ),
                    ));
                }
            }

            if edges.len() == 1 {
                current = edges[0].0;
            } else if edges.len() > 0 {
                let mut probabilities = vec![];
                for edge in &edges {
                    probabilities.push((1.0 / edge.1) / edges.len().as_f64());
                }
                let dist = WeightedIndex::new(&probabilities).unwrap();
                current = edges[dist.sample(&mut self.rng)].0;
            } else {
                let random_pos = self.rng.random_range(0..unvisited.len());
                current = unvisited[random_pos];
            }

            unvisited.retain(|&x| x != current);
            path.push(current);
        }

        let distance = Self::calculate_path_distance(&path, &self.distance_matrix);
        Chromossome::new(path, distance)
    }

    fn genetic_edge_recombination_crossover(
        &mut self,
        parent_1: &Chromossome,
        parent_2: &Chromossome,
    ) -> Chromossome {
        self.crossover = "genetic_edge_recombination_crossover".to_string();
        let parent_1_path = parent_1.get_path();
        let parent_2_path = parent_2.get_path();
        let n = parent_1_path.len();

        let mut path = vec![];
        let mut unvisited = parent_1_path.clone();

        path.push(unvisited[self.rng.random_range(0..unvisited.len() - 1)]);
        let mut current = path[0];
        unvisited.retain(|&x| x != current);

        let mut edge_map: HashMap<u16, HashSet<u16>> = HashMap::new();
        for parent in [parent_1_path, parent_2_path] {
            for i in 0..parent.len() {
                let city = parent[i];
                let set = edge_map.entry(city).or_insert_with(HashSet::new);

                if i + 1 == parent.len() {
                    set.insert(parent[0]);
                } else {
                    set.insert(parent[i + 1]);
                }

                if i == 0 {
                    set.insert(parent[parent.len() - 1]);
                } else {
                    set.insert(parent[i - 1]);
                }
            }
        }

        while path.len() < n {
            for edge_set in edge_map.values_mut() {
                edge_set.remove(&current);
            }

            let current_set = edge_map.get(&current).unwrap();
            if !current_set.is_empty() {
                let less_cities = current_set
                    .iter()
                    .min_by(|c1, c2| {
                        edge_map
                            .get(c1)
                            .unwrap()
                            .len()
                            .cmp(&edge_map.get(c2).unwrap().len())
                    })
                    .unwrap_or(&unvisited[self.rng.random_range(0..unvisited.len())]);
                current = less_cities.clone();
            } else {
                current = unvisited[self.rng.random_range(0..unvisited.len())];
            }
            unvisited.retain(|&x| x != current);
            path.push(current);
        }

        let distance = Self::calculate_path_distance(&path, &self.distance_matrix);
        Chromossome::new(path, distance)
    }

    fn maximal_preservative_crossover(
        &mut self,
        parent_1: &Chromossome,
        parent_2: &Chromossome,
    ) -> Chromossome {
        self.crossover = "maximal_preservative_crossover".to_string();

        let n = parent_1.get_path().len();

        let start = self.rng.random_range(0..n - 10);
        let end;
        if start + 10 < n / 2 {
            end = self.rng.random_range(start + 10..n / 2);
        } else {
            end = n - 1;
        }

        let mut path = parent_1.get_path()[start..end].to_vec();

        for &city in parent_2.get_path() {
            if !path.contains(&city) {
                path.push(city);
            }
        }

        let distance = Self::calculate_path_distance(&path, &self.distance_matrix);
        Chromossome::new(path, distance)
    }

    fn partially_mapped_crossover(
        &mut self,
        parent_1: &Chromossome,
        parent_2: &Chromossome,
    ) -> Chromossome {
        self.crossover = "partially_mapped_crossover".to_string();

        let n = parent_1.get_path().len();

        let start = self.rng.random_range(0..n - 2);
        let end = self.rng.random_range(start + 1..n - 1);

        let mut path = parent_1.get_path().clone();
        let initial = parent_1.get_path()[start..end].to_vec();
        let replace = parent_2.get_path()[start..end].to_vec();

        path[start..end].copy_from_slice(&replace);

        let duplicated: Vec<_> = replace
            .iter()
            .filter(|x| !initial.contains(x))
            .cloned()
            .collect();

        let mut to_replace: Vec<_> = initial
            .iter()
            .filter(|x| !replace.contains(x))
            .rev()
            .cloned()
            .collect();

        let mut i = 0;
        for city in path.clone() {
            if duplicated.contains(&city) && !(start..end).contains(&i) {
                path[i] = to_replace.pop().unwrap();
            }
            i += 1;
        }

        let distance = Self::calculate_path_distance(&path, &self.distance_matrix);
        Chromossome::new(path, distance)
    }

    fn alternating_position_crossover(
        &mut self,
        parent_1: &Chromossome,
        parent_2: &Chromossome,
    ) -> Chromossome {
        self.crossover = "alternating_position_crossover".to_string();

        let n = parent_1.get_path().len();
        let mut path = vec![];
        let mut i = 0;
        while path.len() < n {
            if !path.contains(&parent_1.get_path()[i]) {
                path.push(parent_1.get_path()[i]);
            }

            if !path.contains(&parent_2.get_path()[i]) {
                path.push(parent_2.get_path()[i]);
            }

            i += 1;
        }

        let distance = Self::calculate_path_distance(&path, &self.distance_matrix);
        Chromossome::new(path, distance)
    }

    fn cycle_crossover_v2(
        &mut self,
        parent_1: &Chromossome,
        parent_2: &Chromossome,
    ) -> Chromossome {
        self.crossover = "cycle_crossover_v2".to_string();
        let parent_1_path = parent_1.get_path();
        let parent_2_path = parent_2.get_path();

        let n = parent_1_path.len();
        let mut path = vec![];

        let mut availiable = parent_2_path[0..n].to_vec();

        let mut current;

        while availiable.len() > 0 {
            let mut turn = 2;
            let mut cycle = vec![];
            current = availiable[0];

            loop {
                if turn % 2 == 0 {
                    if cycle.contains(&current) || !availiable.contains(&current) {
                        break;
                    }
                    cycle.push(current);
                    availiable.retain(|&x| x != current as u16);
                    path.push(current);
                }
                let i = parent_1_path.iter().position(|&x| x == current).unwrap();
                current = parent_2_path[i];
                turn += 1;
            }
        }

        let distance = Self::calculate_path_distance(&path, &self.distance_matrix);
        Chromossome::new(path, distance)
    }

    fn execute_for_population(
        &mut self,
        mut population: Vec<Chromossome>,
        initial_swap: usize,
    ) -> Vec<Chromossome> {
        let mut worst = self.get_worst_chromossome(&population);
        let mut best = self.get_best_chromossome(&population);
        let mut gen_not_changed_best = 0;
        let mut gen_not_changed_best_limit = self.cities.len();
        let gen_not_changed_best_breakpoint = 200000;
        let mut swap = initial_swap;
        while gen_not_changed_best < gen_not_changed_best_breakpoint {
            let (parent_1, parent_2) = self.select_parents(&population);
            let children = self
                .partially_mapped_crossover(&parent_1, &parent_2)
                .mutate(&self.distance_matrix, swap);

            if children.get_distance() < worst.get_distance() {
                let i = population.iter().position(|x| x == &worst).unwrap();

                population.remove(i);
                population.push(children.clone());

                worst = self.get_worst_chromossome(&population);

                if children.get_distance() < best.get_distance() {
                    best = children;
                    let mutation = best.get_mutation();
                    self.mutations.insert(mutation.clone());
                    // println!(
                    //     "{} {} {} {}",
                    //     &best.get_distance(),
                    //     self.generations,
                    //     swap,
                    //     mutation
                    // );
                    if swap > 1 {
                        swap -= 1;
                    }
                    gen_not_changed_best = 0;
                } else {
                    gen_not_changed_best += 1;
                }
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

        population
    }

    fn execute_for_one_population_army(
        &mut self,
        first_gen: &Chromossome,
        initial_swap: usize,
    ) -> Chromossome {
        let mut previous_distance = first_gen.get_distance().clone();
        let mut gen_not_changed_best = 0;
        let mut gen_not_changed_best_limit = self.cities.len();
        let gen_not_changed_best_breakpoint = 500000;
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
                // println!(
                //     "{} {} {} {}",
                //     &new_distance, self.generations, swap, mutation
                // );
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

impl Algorithm for Genetic {
    fn execute(&mut self) -> ExecuteResponse {
        println!("Execute Genetic");
        let start_time = Instant::now();
        let len_cities = self.cities.len();
        let greedy_range = ((len_cities as f32).sqrt() as usize).max(10);
        let population_size;
        if len_cities > 1000 {
            population_size = 1;
        } else {
            population_size = 100;
        }
        self.distance_matrix = Self::create_distance_matrix(&self.cities);

        let mut population = self.create_random_population(population_size);
        // let mut population = self.create_greedy_population(population_size, greedy_range);

        let first_gen_best_path = population[0].get_path().clone();

        let swap = 1;

        if population_size == 1 {
            population[0] = self.execute_for_one_population_army(&population[0], swap);
        } else {
            population = self.execute_for_population(population, swap);
        }

        let metadata = format!(
            "Population Size: {}\nGenerations: {}\nCrossover: {}\nMutations: {:?}\n",
            population_size, self.generations, self.crossover, self.mutations
        );
        // let metadata = format!(
        //     "Population Size: {}\nGenerations: {}\nCrossover: {}\nMutations: {:?}\nGreedy Start Range: {}",
        //     population_size, self.generations, self.crossover, self.mutations, greedy_range
        // );

        let best = self.get_best_chromossome(&population);

        ExecuteResponse::new(
            first_gen_best_path,
            best.get_path().clone(),
            best.get_distance().clone(),
            start_time.elapsed(),
            metadata,
        )
    }
}
