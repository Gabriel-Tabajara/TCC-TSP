mod algorithm;
mod models;

use algorithm::algorithm_strategy::AlgorithmStrategy;
use clap::Parser;
use core::f32;
use csv::Reader;
use models::{
    city::City,
    coordinates::Coordinates,
    graph_metadata::GraphMetadata,
    uf::{UF, UFEnum},
};
use plotters::{
    chart::ChartBuilder,
    prelude::{BitMapBackend, Circle, IntoDrawingArea, PathElement},
    style::{BLUE, Color, RED, WHITE},
};
use std::{
    error::Error,
    fs::{File, create_dir_all},
    iter::once,
    path::Path,
};

#[derive(Parser)]
#[command(name = "Optimizer")]
struct Args {
    #[arg(short = 'a', long, default_value = "G")]
    algorithm: String,

    #[arg(short = 'u', long, default_value = "BRAZIL")]
    uf: String,

    #[arg(short = 'p', long)]
    plot: bool,
}

fn plot_state(
    cities: &Vec<City>,
    cities_path: &[u16],
    file_path: &str,
    uf: &UF,
) -> Result<(), Box<dyn Error>> {
    let (min_x, max_x): (f32, f32) = uf.get_min_max_longitude().clone();
    let (min_y, max_y): (f32, f32) = uf.get_min_max_latitude().clone();
    let image_size = (1024, 768);
    let font_style = "sans-serif";
    let caption_font_size = 30;
    let y_x_font_size = 20;

    create_dir_all(Path::new(file_path).parent().unwrap())?;

    let image = BitMapBackend::new(file_path, image_size).into_drawing_area();
    image.fill(&WHITE)?;

    let latitudes_array: Vec<f32> = cities
        .iter()
        .map(|city| city.get_coordinates().get_latitude())
        .collect();

    let longitudes_array: Vec<f32> = cities
        .iter()
        .map(|city| city.get_coordinates().get_longitude())
        .collect();

    let path_lines: Vec<(f32, f32)> = cities_path
        .iter()
        .map(|&id| {
            let coordinates = cities[id as usize].get_coordinates();
            (coordinates.get_longitude(), coordinates.get_latitude())
        })
        .collect();

    let mut chart = ChartBuilder::on(&image)
        .caption("Current State Graph", (font_style, caption_font_size))
        .margin(10)
        .x_label_area_size(70)
        .y_label_area_size(70)
        .build_cartesian_2d(min_x - 1.0..max_x + 1.0, min_y - 1.0..max_y + 1.0)?;

    chart
        .configure_mesh()
        .x_desc("Longitude")
        .x_label_style((font_style, y_x_font_size))
        .y_desc("Latitude")
        .y_label_style((font_style, y_x_font_size))
        .draw()?;

    chart.draw_series(
        longitudes_array
            .iter()
            .zip(latitudes_array.iter())
            .map(|(&x, &y)| Circle::new((x, y), 2, RED.filled())),
    )?;

    chart.draw_series(once(PathElement::new(path_lines, &BLUE)))?;

    image.present()?;
    Ok(())
}

fn read_csv_cities(path: &str, uf: &UF) -> Vec<City> {
    let file = File::open(path.to_string()).expect("Failed to open file");
    let mut reader = Reader::from_reader(file);

    let records: Vec<_> = reader.records().collect();

    let mut cities: Vec<City> = Vec::with_capacity(records.len());

    let mut id: u16 = 0;
    for record in records.iter() {
        match record {
            Ok(city) => {
                let city_uf = UF::get_uf_from_code(city[5].parse::<u8>().unwrap()).unwrap();
                if uf.get_uf_enum() == city_uf.get_uf_enum() || *uf.get_uf_enum() == UFEnum::BRAZIL
                {
                    cities.push(City::new(
                        id,
                        UF::get_uf_from_code(city[5].parse::<u8>().unwrap()).unwrap(),
                        Coordinates::new(
                            city[2].parse::<f32>().unwrap(),
                            city[3].parse::<f32>().unwrap(),
                        ),
                    ));
                    id += 1;
                }
            }
            Err(err) => {
                eprintln!("Error reading city in {} file: {}", path, err);
            }
        }
    }

    cities
}

fn main() {
    let args = Args::parse();
    let algorithm = args.algorithm.as_str();
    let uf = UF::get_uf_from_str(args.uf.as_str()).unwrap();
    let plot = args.plot;

    let cities = read_csv_cities("src/assets/cities.csv", &uf);

    let cities_result = AlgorithmStrategy::execute_algorithm(algorithm, &cities);

    if plot {
        let folder = format!(
            "src/assets/results/{}/{}",
            args.uf.as_str().to_uppercase(),
            algorithm
        );

        let mut best_distance_in_file = 0.0;
        let mut best_time_in_file = 0.0;
        let metadata_path = format!("{}/metadata.txt", folder);
        let folder_exists = Path::new(&metadata_path).exists();
        if folder_exists {
            best_distance_in_file = GraphMetadata::get_distance_from_file(&metadata_path);
            best_time_in_file = GraphMetadata::get_time_from_file(&metadata_path);
        }

        if !folder_exists
            || (best_distance_in_file > *cities_result.get_distance())
            || (best_distance_in_file == *cities_result.get_distance())
                && best_time_in_file > cities_result.get_total_time().as_secs_f64()
        {
            let mut initial_path = cities_result.get_initial_path().clone();
            initial_path.push(initial_path[0]);

            let mut final_path = cities_result.get_final_path().clone();
            final_path.push(final_path[0]);

            plot_state(
                &cities,
                &final_path,
                format!("{}/final.png", folder).as_str(),
                &uf,
            )
            .unwrap();

            plot_state(
                &cities,
                &initial_path,
                format!("{}/inicial.png", folder).as_str(),
                &uf,
            )
            .unwrap();

            let metadata = GraphMetadata::new(
                final_path,
                cities_result.get_distance().clone(),
                cities_result.get_total_time().clone(),
                cities_result.get_metadata_info().clone(),
            );

            metadata.generate_file(format!("{}/metadata.txt", folder));
        }
    }

    println!("{}", cities_result.get_distance());
    println!("{:#?}", cities_result.get_total_time().as_secs_f64());
}
