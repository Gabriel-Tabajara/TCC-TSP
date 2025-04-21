use crate::models::city::City;

pub struct ExecuteResponse {
    initial_path: Vec<u16>,
    final_path: Vec<u16>,
    distance: f64
}

impl ExecuteResponse {
    pub fn new(initial_path: Vec<u16>, final_path: Vec<u16>, distance: f64) -> Self {
        ExecuteResponse { initial_path, final_path, distance }
    }

    pub fn get_initial_path (&self) -> &Vec<u16> {
        &self.initial_path
    }

    pub fn get_final_path (&self) -> &Vec<u16> {
        &self.final_path
    }

    pub fn get_distance (&self) -> &f64 {
        &self.distance
    }
}

pub trait Algorithm {
    fn execute(cities: &Vec<City>) -> ExecuteResponse;

    fn calculate_path_distance(path: &[u16], distance_matrix: &[f64]) -> f64 {
        let mut distance: f64 = 0.0;
        let n = path.len();
        for i in 0..n-1 {
            distance += Self::get_in_matrix(distance_matrix, n, path[i] as usize, path[i+1] as usize);
        }
        distance += Self::get_in_matrix(distance_matrix, n, path[n-1] as usize, path[0] as usize);
        //Result is in degress
        distance
    }

    fn calculate_distance_between_cities(city1: &City, city2: &City) -> f64 {
        let c1_coord = city1.get_coordinates();
        let c2_coord = city2.get_coordinates();

        let x_part: f64 = (c2_coord.get_longitude() - c1_coord.get_longitude()).powi(2).into();
        let y_part: f64 = (c2_coord.get_latitude() - c1_coord.get_latitude()).powi(2).into();

        (x_part + y_part).sqrt()
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

    fn matrix_index(size: usize, row: usize, column: usize) -> usize{
        row * size + column
    }
}