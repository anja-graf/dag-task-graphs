use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "task_graphs",
    about = "
The task_graphs tool generates random task graphs using one of two variants of the Erdős–Rényi model:
    G(n, p) – a graph with n nodes where each edge is included with independent probability p.
    G(n, m) – a graph with n nodes and m edges chosen uniformly at random.
For more information on the Erdős–Rényi model, see also https://en.wikipedia.org/wiki/Erd%C5%91s%E2%80%93R%C3%A9nyi_model"
)]

/// Helper class for parsing program arguments
pub struct Args {
    #[arg(short, long, help = "Number of nodes")]
    pub nodes: i32,

    #[arg(short = 'm', long, default_value = "2", help = "Number of edges")]
    pub edges: i32,

    #[arg(short, long, default_value = "0.5", help = "Probability for edges")]
    pub probability: f64,

    #[arg(
        short,
        long,
        default_value = "graph",
        help = "Path for output file in dot format"
    )]
    pub dot: String,

    #[arg(
        short,
        long,
        default_value = "graph",
        help = "Path for output file in svg format"
    )]
    pub svg: String,

    #[arg(
        short = 'P',
        long,
        default_value = "graph",
        help = "Path for output file in png format"
    )]
    pub png: String,

    #[arg(
        short,
        long,
        help = "Path for parameter output file in csv format [default: Not generated]"
    )]
    pub csv: Option<String>,

    #[arg(short = 'g', long, default_value = "1", help = "Number of graphs")]
    pub graphs: i32,

    #[arg(
        short = 't',
        long = "uunifast",
        value_name = "UTILIZATION",
        help = "Total processor utilization for uunifast [default: Not used]"
    )]
    pub uunifast_utilization: Option<f64>,

    #[arg(
        short = 'T',
        long = "drs",
        value_name = "UTILIZATION",
        help = "Total processor utilization for drs [default: Not used]"
    )]
    pub drs_utilization: Option<f64>,

    #[arg(
        short = 'U',
        long,
        value_delimiter = ',',
        value_name = "U1,U2,...,UN",
        help = "Sequence with upper bounds for each node [default: Not used]"
    )]
    pub upper_bounds: Option<Vec<f64>>,

    #[arg(
        short = 'L',
        value_delimiter = ',',
        value_name = "L1,L2,...,LN",
        long,
        help = "Sequence with lower bounds for each node [default: Not used]"
    )]
    pub lower_bounds: Option<Vec<f64>>,

    #[arg(
        short = 'D',
        long,
        help = "Path for drs input file [default: Not used]"
    )]
    pub drs_path: Option<String>,

    #[arg(
        short,
        long,
        default_value = "1",
        help = "Variant selector 1 for G(n,p) or 2 for G(n,m)"
    )]
    pub option: i32,
}
