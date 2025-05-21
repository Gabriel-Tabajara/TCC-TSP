use crate::models::city::City;
use std::time::Duration;

#[derive(Clone)]
pub struct ExecuteResponse {
    initial_path: Vec<u16>,
    final_path: Vec<u16>,
    distance: f64,
    total_time: Duration,
    metadata_info: String,
}

impl ExecuteResponse {
    pub fn new(
        initial_path: Vec<u16>,
        final_path: Vec<u16>,
        distance: f64,
        total_time: Duration,
        metadata_info: String,
    ) -> Self {
        ExecuteResponse {
            initial_path,
            final_path,
            distance,
            total_time,
            metadata_info,
        }
    }

    pub fn get_initial_path(&self) -> &Vec<u16> {
        &self.initial_path
    }

    pub fn get_final_path(&self) -> &Vec<u16> {
        &self.final_path
    }

    pub fn get_distance(&self) -> &f64 {
        &self.distance
    }

    pub fn get_total_time(&self) -> &Duration {
        &self.total_time
    }

    pub fn get_metadata_info(&self) -> &String {
        &self.metadata_info
    }
}

pub trait Algorithm {
    fn execute(&mut self) -> ExecuteResponse;

    fn calculate_path_distance(path: &[u16], distance_matrix: &[f64]) -> f64 {
        let mut distance: f64 = 0.0;
        let n = path.len();
        for i in 0..n - 1 {
            distance +=
                Self::get_in_matrix(distance_matrix, n, path[i] as usize, path[i + 1] as usize);
        }
        distance += Self::get_in_matrix(distance_matrix, n, path[n - 1] as usize, path[0] as usize);
        //Result is in degress
        distance
    }

    fn calculate_distance_between_cities(city1: &City, city2: &City) -> f64 {
        let c1_coord = city1.get_coordinates();
        let c2_coord = city2.get_coordinates();

        let x_part: f64 = (c2_coord.get_longitude() - c1_coord.get_longitude())
            .powi(2)
            .into();
        let y_part: f64 = (c2_coord.get_latitude() - c1_coord.get_latitude())
            .powi(2)
            .into();

        (x_part + y_part).sqrt()
    }

    fn calculate_distance_between_cities_ids(id1: usize, id2: usize, cities: &Vec<City>) -> f64 {
        Self::calculate_distance_between_cities(&cities[id1], &cities[id2])
    }

    fn find_best_neighbour(
        distance_matrix: &[f64],
        id_city: usize,
        n: usize,
        filter: &[usize],
    ) -> usize {
        let distance_array = Self::get_entire_row_in_matrix(distance_matrix, n, id_city);
        distance_array
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != id_city && !filter.contains(&i))
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap()
    }

    fn find_n_best_neighbours(
        distance_matrix: &[f64],
        id_city: usize,
        cities_len: usize,
        n: usize,
    ) -> Vec<usize> {
        Self::find_n_best_neighbours_with_filter(distance_matrix, id_city, cities_len, n, &[])
    }

    fn find_n_best_neighbours_with_filter(
        distance_matrix: &[f64],
        id_city: usize,
        cities_len: usize,
        n: usize,
        filter: &[usize],
    ) -> Vec<usize> {
        let distance_array = Self::get_entire_row_in_matrix(distance_matrix, cities_len, id_city);
        let mut connections_tuple: Vec<(usize, &f64)> = distance_array
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != id_city && !filter.contains(&i))
            .collect();

        connections_tuple.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

        connections_tuple.iter().take(n).map(|(i, _)| *i).collect()
    }

    fn create_distance_matrix(cities: &Vec<City>) -> Vec<f64> {
        let n = cities.len();

        let mut distance_matrix: Vec<f64> = vec![0.0; n * n];

        for i in 0..n {
            for j in 0..n {
                if i < j {
                    let distance = Self::calculate_distance_between_cities(&cities[i], &cities[j]);
                    distance_matrix[Self::matrix_index(n, i, j)] = distance;
                    distance_matrix[Self::matrix_index(n, j, i)] = distance;
                }
            }
        }

        distance_matrix
    }

    fn get_in_matrix(matrix: &[f64], size: usize, row: usize, column: usize) -> f64 {
        matrix[Self::matrix_index(size, row, column)]
    }

    fn get_entire_row_in_matrix(matrix: &[f64], size: usize, row: usize) -> Vec<f64> {
        let start = row * size;
        let end = start + size;
        matrix[start..end].to_vec()
    }

    fn matrix_index(size: usize, row: usize, column: usize) -> usize {
        row * size + column
    }

    fn print_distance_matrix(distance_matrix: &[f64], size: usize) {
        for i in 0..size {
            for j in 0..size {
                print!("{:.2} ", distance_matrix[Self::matrix_index(size, i, j)]);
            }
            println!();
        }
    }
}
