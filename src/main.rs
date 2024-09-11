use rand::Rng;
use std::fs::File;
use std::io::{BufWriter, Write};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "AlphaBetaRookGraph Generator")]
struct Cli {
    /// Width of the grid (N)
    #[structopt(short, default_value = "10")]
    n: usize,

    /// Height of the grid (M)
    #[structopt(short, default_value = "10")]
    m: usize,

    /// Alpha parameter for edge addition probability
    #[structopt(short="a", long, default_value = "0.1")]
    alpha: f64,

    /// Beta parameter for edge removal probability
    #[structopt(short="b", long, default_value = "0.1")]
    beta: f64,

    /// Output file name
    #[structopt(default_value = "graph.gr")]
    output_file: String,
}

struct AlphaBetaRookGraph {
    n: usize,
    m: usize,
    alpha: f64,
    beta: f64,
    adj_matrix: Vec<Vec<bool>>,
}

impl AlphaBetaRookGraph {
    fn new(n: usize, m: usize, alpha: f64, beta: f64) -> Self {
        let graph_size = n * m;
        let mut graph = AlphaBetaRookGraph {
            n,
            m,
            alpha,
            beta,
            adj_matrix: vec![vec![false; graph_size]; graph_size],
        };
        graph.generate_cliques();
        graph.modify_edges();
        graph
    }

    fn generate_cliques(&mut self) {
        // Create row and column cliques
        for i in 0..self.n {
            for j in 0..self.m {
                for k in j + 1..self.m {
                    self.adj_matrix[i * self.m + j][i * self.m + k] = true;
                    self.adj_matrix[i * self.m + k][i * self.m + j] = true;
                }
            }
        }
        for j in 0..self.m {
            for i in 0..self.n {
                for k in i + 1..self.n {
                    self.adj_matrix[i * self.m + j][k * self.m + j] = true;
                    self.adj_matrix[k * self.m + j][i * self.m + j] = true;
                }
            }
        }
    }

    fn modify_edges(&mut self) {
        let mut rng = rand::thread_rng();
        let graph_size = self.n * self.m;

        // Modify edges based on alpha and beta
        for i in 0..graph_size {
            for j in (i + 1)..graph_size {
                let row_i = i / self.m;
                let col_i = i % self.m;
                let row_j = j / self.m;
                let col_j = j % self.m;

                if row_i == row_j || col_i == col_j {
                    // Edge is part of a row or column clique, possibly remove
                    if rng.gen::<f64>() < self.beta {
                        self.adj_matrix[i][j] = false;
                        self.adj_matrix[j][i] = false;
                    }
                } else {
                    // Edge is not in the same row or column, possibly add
                    if rng.gen::<f64>() < self.alpha {
                        self.adj_matrix[i][j] = true;
                        self.adj_matrix[j][i] = true;
                    }
                }
            }
        }
    }

    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);

        let n = self.n * self.m;
        let mut edges = Vec::new();

        // Collect all edges (no duplicates, only add u -> v for u < v to avoid duplicates)
        for u in 0..n {
            for v in (u + 1)..n {
                if self.adj_matrix[u][v] {
                    edges.push((u + 1, v + 1)); // 1-indexed
                }
            }
        }

        // Write the header line
        writeln!(writer, "p tww {} {}", n, edges.len())?;

        // Write the edges
        for (u, v) in edges {
            writeln!(writer, "{} {}", u, v)?;
        }

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let args = Cli::from_args();

    let graph = AlphaBetaRookGraph::new(args.n, args.m, args.alpha, args.beta);
    graph.save_to_file(&args.output_file)?;

    println!(
        "Alpha-Beta-Rook-Graph generated with n='{}' m='{}' α='{}' β='{}' and saved to '{}'.",
        args.n, args.m, args.alpha, args.beta, args.output_file
    );
    Ok(())
}
