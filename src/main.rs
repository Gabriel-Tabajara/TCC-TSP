mod models;
mod algorithm;

use models::{coordinates::Coordinates, city::City};
use std::{fs::File, error::Error, env};
use algorithm::algorithm_strategy::AlgorithmStrategy;
use csv::Reader;
// update to just the used ones
use plotters::prelude::*;
 
fn plot_current_state(cities: &Vec<City>, path: &str) -> Result<(), Box<dyn Error>> {
    let (min_x, max_x): (f32, f32) = (-75.0, -35.0);
    let (min_y, max_y): (f32, f32) = (-33.0, 5.0);
    let image_size = (1024, 768);
    let font_style = "sans-serif";
    let caption_font_size = 30;
    let y_x_font_size = 20;

    let image = BitMapBackend::new(path, image_size).into_drawing_area();
    image.fill(&WHITE)?;

    let latitudes_array: Vec<f32> = cities.iter()
        .map(|city| city.get_coordinates().get_latitude())
        .collect();

    let longitudes_array: Vec<f32> = cities.iter()
        .map(|city| city.get_coordinates().get_longitude())
        .collect();

    let mut chart = ChartBuilder::on(&image)
        .caption("Current State Graph", (font_style, caption_font_size))
        .margin(10)
        .x_label_area_size(70)
        .y_label_area_size(70)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    chart.configure_mesh()
        .x_desc("Longitude")
        .x_label_style((font_style, y_x_font_size))
        .y_desc("Latitude")
        .y_label_style((font_style, y_x_font_size))
        .draw()?;

    chart.draw_series(
        longitudes_array.iter().zip(latitudes_array.iter()).map(|(&x, &y)| {
            Circle::new((x, y), 2, RED.filled())
        }),
    )?;

    image.present()?;
    Ok(())
}

fn read_csv_file(path: &str) -> Vec<City> {
    let file = File::open(path.to_string()).expect("Failed to open file");
    let mut reader = Reader::from_reader(file);

    let records: Vec<_> = reader.records().collect();

    let mut cities: Vec<City> = Vec::with_capacity(records.len());

    let mut id: u16 = 0;
    for record in records.iter() {
        match record {
            Ok(city) => {
                cities.push(
                    City::new(
                        id, 
                        Coordinates::new(
                            city[2].parse::<f32>().unwrap(),
                            city[3].parse::<f32>().unwrap()
                    )
                    )
                );
            }
            Err(err) => {
                eprintln!("Error reading city in {} file: {}", path, err);
            }
        }
        id += 1;
    }

    cities
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let cities = read_csv_file("src/assets/cities.csv");
    plot_current_state(&cities, "src/assets/graph.png").unwrap();
    
    let cities_result = AlgorithmStrategy::execute_algorithm(args[1].as_str(), cities);
}
