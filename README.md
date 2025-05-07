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

## Genetic

### Mutations

- `swap_mutation` - Swaps the positions of two randomly selected cities in the tour.
- `displacement_mutation` - Removes a random subsequence (subtour) from the path and reinserts it at a different random position.
- `insertion_mutation` - Selects a city and reinserts it at another random position in the path.
- `simple_inversion_mutation` - Reverses the order of a randomly selected subsequence (subtour) within the path.
- `inversion_mutation` - Selects a subsequence, reverses it, and reinserts it at a different random position in the tour.
- `greedy_sub_tour_mutation` - Removes a subsequence and reinserts it at the position that minimizes the total tour length increase (greedy reinsertion).
- `greedy_insertion_mutation` - Selects a city and reinserts it in the position among its `N = max(5, sqrt(path.len()))` nearest neighbors.

### Cycle Crossover (CX)

1) Starts with the first city from `parent_1` and look at the city from `parent_2` that is in the same position.
2) Find the `parent_2` looked city in `parent_1` and add it to the child in its `parent_1` position.
3) Repeat the process until all cities from `parent_1` are added to the child or it reaches the first city added to the child.
4) Fills remaining positions in the child with cities from `parent_2`.

## Repository used to find the cities locations

- [kelvins/municipios-brasileiros](https://github.com/kelvins/municipios-brasileiros)