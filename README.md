# AlphaBetaRookGraph Generator

## Overview

The **AlphaBetaRookGraph Generator** is a CLI tool designed to generate a
**rook graph** with a specified width `N` and height `M`. The generated graph
is initially a grid where each row and column forms a clique. The tool then
modifies the graph by adding and removing edges based on provided
probabilities.

## Compilation
To compile the project and create the executable binary, use the following
command:
```bash
cargo build --release
```

## Run the Binary
```bash
./target/release/alpha_beta_rook_graph_generator -n 15 -m 20 -a 0.2 -b 0.3 graph_output.gr
```

### Options
- -n (default: 10): Width of the grid.
- -m (default: 10): Height of the grid.
- -a, --alpha (default: 0.1): Probability of adding edges between vertices not in the same row or column.
- -b, --beta (default: 0.1): Probability of removing edges between vertices in the same column.
- output (default: graph.gr): Name of the output file where the graph will be saved.

