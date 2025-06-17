use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "erdosrenyimodel",
    about = "
The erdosrenyimodel tool generates a random task graph using one of two variants of the Erdős–Rényi model:
    G(n, p) – a graph with n nodes where each edge is included with independent probability p.
    G(n, m) – a graph with n nodes and m edges chosen uniformly at random.
For more information on the Erdős–Rényi model, see also https://en.wikipedia.org/wiki/Erd%C5%91s%E2%80%93R%C3%A9nyi_model"
)]

/// Helper class for parsing program arguments
pub struct Args {
    #[arg(short, long, help = "Number of nodes")]
    pub nodes: i32,
    #[arg(short='m', long, default_value = "2", help = "Number of edges")]
    pub edges: i32,

    #[arg(short, long, default_value = "0.5", help = "Probability for edges")]
    pub probability: f64,

    #[arg(short, long, default_value = "graph.dot", help = "Path for output file in dot format")]
    pub dot: String,

    #[arg(short, long, default_value = "graph.svg", help = "Path for output file in svg format")]
    pub svg: String,

    #[arg(short, long, default_value = "1", help = "Variant selector 1 for G(n,p) or 2 for G(n,m)")]
    pub option: i32,
}
