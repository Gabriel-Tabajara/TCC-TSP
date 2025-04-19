# TCC-TSP

## Technologies

- Rust 1.86.0

## How to install Rust

#TBD

## How to Run

`cargo run -q -- --algorithm BB --uf RS --plot`

- default plot is `false`, when `true` it will plot the graph
- default algorithm is `G` (Genetic Algorithm)
  - options are `G` (Genetic Algorithm), `BB` (Branch and Bound), `SA` (Simulated Annealing) and `ADO` (Ant Colony Optimization)
- default uf is `BRAZIL`


- In order to plot the graphs you need to install `sudo apt install pkg-config libfontconfig1-dev`


## Repository used to find the cities locations

- [kelvins/municipios-brasileiros](https://github.com/kelvins/municipios-brasileiros)