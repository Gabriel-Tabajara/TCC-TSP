use std::{
    fs::{read_to_string, write},
    time::{Duration, Instant},
};

pub struct GraphMetadata {
    path: Vec<u16>,
    distance: f64,
    total_time: Duration,
    custom_info: String,
}

impl GraphMetadata {
    pub fn new(path: Vec<u16>, distance: f64, total_time: Duration, custom_info: String) -> Self {
        GraphMetadata {
            path,
            distance,
            total_time,
            custom_info,
        }
    }

    pub fn get_distance_from_file(path: &String) -> f64 {
        let metadata_string = read_to_string(path).unwrap();
        let split = metadata_string.split("\n");

        let mut distance = 0.0;

        for line in split {
            if line.starts_with("Distance: ") {
                distance = line["Distance: ".len()..].trim().parse::<f64>().unwrap();
            }
        }

        distance
    }

    pub fn get_time_from_file(path: &String) -> f64 {
        let metadata_string = read_to_string(path).unwrap();
        let split = metadata_string.split("\n");

        let mut time = 0.0;

        for line in split {
            if line.starts_with("Total Time: ") {
                time = line["Total Time: ".len()..].trim().parse::<f64>().unwrap();
            }
        }

        time
    }

    pub fn generate_file(&self, path: String) {
        let file_text = format!(
            "Path: {:?}\nDistance: {}\nTotal Time: {}\n{}",
            &self.path,
            &self.distance,
            &self.total_time.as_secs_f64(),
            &self.custom_info
        );
        write(path, file_text).unwrap();
    }
}
