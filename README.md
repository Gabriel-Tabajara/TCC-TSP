# TCC - Trabalho de Conclusão de Curso

## Students 
- Gabriel Tabajara dos Santos
- Giovanni Masoni Schenato

## Overview
This project is a Rust implementation of the Traveling Salesman Problem (TSP) using various algorithms such as Genetic Algorithm, Branch and Bound, Simulated Annealing, and Ant Colony Optimization.

## Technologies

- Rust 1.86.0

## How to Run

`cargo run -q -- --algorithm BB --uf RS --plot`

- default plot is `false`, when `true` it will plot the graph
- default algorithm is `G` (Genetic Algorithm)
  - options are `G` (Genetic Algorithm), `BB` (Branch and Bound), `SA` (Simulated Annealing) and `ACO` (Ant Colony Optimization)
- default uf is `BRAZIL`

- In order to plot the graphs you need to install `sudo apt install pkg-config libfontconfig1-dev`

## Running with run.sh

You can run the project using the provided `run.sh` script. This script allows you to run the same configuration for several test cases with just one command.

## Repository used to find the cities locations

- [kelvins/municipios-brasileiros](https://github.com/kelvins/municipios-brasileiros)