mod models;
mod algorithm;

use models::{city::City, coordinates::Coordinates, uf::{UFEnum, UF}};
use clap::Parser;
use core::f32;
use std::{fs::File, error::Error};
use algorithm::algorithm_strategy::AlgorithmStrategy;
use csv::Reader;
// update to just the used ones
use plotters::prelude::*;

#[derive(Parser)]
#[command(name = "Optimizer")]
struct Args {
    #[arg(short='a', long, default_value="G")]
    algorithm: String,

    #[arg(short='u', long, default_value="BRAZIL")]
    uf: String,

    #[arg(short='p', long)]
    plot: bool
}
 
fn plot_current_state(cities: &Vec<City>, path: &str, uf: &UF) -> Result<(), Box<dyn Error>> {
    let (min_x, max_x): (f32, f32) = uf.get_min_max_longitude().clone();
    let (min_y, max_y): (f32, f32) = uf.get_min_max_latitude().clone();
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
        .build_cartesian_2d(min_x-1.0..max_x+1.0, min_y-1.0..max_y+1.0)?;

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
                        UF::get_uf_from_code(
                            city[5].parse::<u8>().unwrap()
                        ).unwrap(),
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

fn retrieve_cities_for_uf(uf: &UFEnum, cities: &Vec<City>) -> Vec<City> {
    cities.iter().filter(|city| city.get_uf().get_uf_enum() == uf).cloned().collect()
}

fn main() {
    let args = Args::parse();
    let algorithm = args.algorithm.as_str();
    let uf = UF::get_uf_from_str(args.uf.as_str()).unwrap();
    let plot = args.plot;

    let mut cities = read_csv_file("src/assets/cities.csv");

    if *uf.get_uf_enum() != UFEnum::BRAZIL {
        cities = retrieve_cities_for_uf(&uf.get_uf_enum(), &cities);
    }
    
    if plot {
        plot_current_state(&cities, "src/assets/graph.png", &uf).unwrap();
    }
    
    let cities_result = AlgorithmStrategy::execute_algorithm(algorithm, cities);
}
