mod model;

use std::fs;
use model::city::City;
use model::coordinates::Coordinates;
use std::fs::File;
use csv::Reader;
use plotly::Scatter;
use plotly::common::Mode;
use plotly::{Plot, Layout};

fn open_plot_as_html(plot: Plot) {
    let html = plot.to_html(); 

    fs::write("index.html", html).expect("Error while writing to index.html");

    std::process::Command::new("explorer.exe")
        .arg("index.html")
        .status()
        .expect("Failed to open in browser");     
}

fn plot_current_state(cities: &Vec<City>) {
    let latitude_array: Vec<f32> = cities.iter()
                                         .map(|city| city.get_coordinates().get_latitude())
                                         .collect();

    let longitude_array: Vec<f32> = cities.iter()
                                          .map(|city| city.get_coordinates().get_longitude())
                                          .collect();

    let trace = Scatter::new(longitude_array, latitude_array).mode(Mode::Markers).name("TSP");

    let mut plot = Plot::new();
    plot.add_trace(trace);

    let layout = Layout::new()
        .x_axis(plotly::layout::Axis::new().scale_anchor("y"))
        .y_axis(plotly::layout::Axis::new());

    plot.set_layout(layout);

    open_plot_as_html(plot);          
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
    let cities = read_csv_file("./assets/cities.csv");
    plot_current_state(&cities)
}
