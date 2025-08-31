mod drs;
mod erdosrenyi;
mod graph;
mod interface;
mod output;
mod schedule;
mod uunifast;

use drs::DRS;
use graph::Graph;
use uunifast::UUnifast;

use clap::CommandFactory;
use clap::Parser;
use interface::Args;
use rand::Rng;
use std::io::Result;

fn main() -> Result<()> {
    // If no arguments are given, the help will be printed instead of error
    if std::env::args().len() == 1 {
        Args::command().print_help().unwrap();
        std::process::exit(0);
    }

    // Parsing all given arguments and check minimum requirements
    let args = Args::parse();
    assert!(args.nodes > 1, "There should be more than one node!");
    assert!(args.graphs >= 1, "There should be at least one graph!");
    assert!(
        !(args.option == 1 && args.option == 2),
        "There is only option 1 or 2"
    );

    // Generate random node and edge distribution of given inputs for multiple graphs
    let node_distribution: Vec<i32> = generate_random_node_distribution(args.graphs, args.nodes);
    let mut edge_distribution: Vec<i32> = Vec::new();
    if args.option == 2 {
        // Only this variant of erdos renyi takes a number of edges
        edge_distribution = generate_random_edge_distribution(&node_distribution, args.edges);
    }

    let mut all_graphs: Vec<Graph> = Vec::new(); // Here all graphs are stored, so length is equal to args.graphs
    let mut all_periods: Vec<i32> = Vec::new(); // Periods are set to the same value for each graph which is sum of all C_i

    // Generate DAG with variant of erdos renyi model
    generate_random_directed_acyclic_graphs(
        &args,
        &node_distribution,
        &mut edge_distribution,
        &mut all_graphs,
        &mut all_periods,
    );

    // Generate utilization vector with chosen algorithm (uunifast or drs)
    let utilization = generate_utilization(&args, all_periods, &node_distribution);

    let mut offset = 0;
    for (i, graph) in all_graphs.iter_mut().enumerate() {
        // Generate utilization subvector
        let mut sub_u: Vec<f64> = Vec::new(); // Subvector is part of utilization vector with length of nodes in graph i
        if args.uunifast_utilization.is_some() || args.drs_utilization.is_some() {
            sub_u = utilization[offset..offset + node_distribution[i] as usize].to_vec();
            offset += node_distribution[i] as usize; // Offset signals start of U_i values for current graph i
            println!("\n\nSubvector for utilization of graph {}: {:?}", i, sub_u);
        }

        // Find execution order for all tasks in current graph i
        let schedule = graph.distribute_parameters_uniprocessor(&sub_u);

        // Save resulting graph in dot file, svg file and png file and save parameters of all tasks if wanted
        let dot = format!("{}{}.dot", args.dot, i + 1);
        graph.write_dot(&dot, schedule, args.uunifast_utilization.is_some())?;
        if let Some(csv) = &args.csv {
            graph.write_parameters(&format!("{}{}.csv", csv, i + 1))?;
        }
        let svg = &format!("{}{}.svg", args.svg, i + 1);
        let png = &format!("{}{}.png", args.png, i + 1);
        Graph::dot_to_svg_png(&dot, &svg, &png)?;
        println!(
            "Graph {} saved as '{}', '{}' and '{}'",
            i + 1,
            &dot,
            &svg,
            &png
        );
    }

    Ok(())
}

/// Distributes given number of nodes over specified number of graphs,
/// so that each graph has at least one node
/// e.g. generate_random_node_distribution(2, 4) returns [1, 3], [2, 2] or [3, 1]
fn generate_random_node_distribution(graphs: i32, nodes: i32) -> Vec<i32> {
    assert!(
        nodes >= graphs,
        "There should be at least one node for each graph"
    );
    let mut distribution = vec![1; graphs as usize];
    let mut rng = rand::thread_rng();
    let rest = nodes - graphs;
    for _ in 0..rest {
        distribution[rng.gen_range(0..graphs) as usize] += 1;
    }
    println!("Nodes were distributed on graphs: {:?}", distribution);
    distribution
}

/// Distributes given number of edges over graphs 
/// so that each graph has at most n * (n - 1) / 2 edges
/// e.g. generate_random_node_distribution([1, 3], 2) returns [0, 2]
fn generate_random_edge_distribution(node_distribution: &Vec<i32>, total_edges: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut remaining = total_edges;
    let mut distribution = vec![0; node_distribution.len()];

    // calculate max edges for each graph
    let max_edges: Vec<i32> = node_distribution.iter().map(|&n| n * (n - 1) / 2).collect();

    assert!(
        total_edges <= max_edges.iter().sum(),
        "Edges can't be distributed among the graphs, because number of edges is too high!"
    );

    while remaining > 0 {
        let graph = rng.gen_range(0..node_distribution.len());
        if distribution[graph] < max_edges[graph] {
            distribution[graph] += 1;
            remaining -= 1;
        }
    }
    println!("Edges were distributed on graphs: {:?}", distribution);
    distribution
}

/// Calls for each generated graph erdos renyi function to generate a directed acyclic graph
fn generate_random_directed_acyclic_graphs(
    args: &Args,
    node_distribution: &Vec<i32>,
    edge_distribution: &mut Vec<i32>,
    all_graphs: &mut Vec<Graph>,
    all_periods: &mut Vec<i32>,
) {
    for i in 0..args.graphs {
        // Depending on the specified option a function is used to generate the graph
        let graph;
        if args.option == 1 {
            println!(
                "The graph {} will be generated with option 1, nodes = {} and probability = {}",
                i + 1,
                node_distribution[i as usize],
                args.probability
            );
            graph = Graph::new_erdos_renyi_var1(node_distribution[i as usize], args.probability);
        } else if args.option == 2 {
            println!(
                "The graph {} will be generated with option 2, nodes = {} and edges = {}",
                i + 1,
                node_distribution[i as usize],
                edge_distribution[i as usize]
            );
            graph = Graph::new_erdos_renyi_var2(
                node_distribution[i as usize],
                edge_distribution[i as usize],
            );
        } else {
            panic!("Error: Invalid option!");
        }

        if let Some(first_node) = graph.nodes.first() {
            // all graphs have at least one node
            all_periods.push(first_node.period); // T_1 = T_2 = ... = T_n
        }

        all_graphs.push(graph);
    }
}

// Generates task utilization with uunifast or drs algorithm for all graphs 
fn generate_utilization(
    args: &Args,
    all_periods: Vec<i32>,
    node_distribution: &Vec<i32>,
) -> Vec<f64> {
    let mut utilization: Vec<f64> = Vec::new(); // Here all task utilizations are stored, so length is equal to args.nodes
    if let Some(u) = &args.uunifast_utilization {
        utilization = UUnifast::call_uunifast(args.nodes, *u);
    } else if let Some(u) = &args.drs_utilization {
        let mut lower = args.lower_bounds.clone();
        let mut upper = args.upper_bounds.clone();
        // If a drs input file is given, read it
        if let Some(path) = &args.drs_path {
            (lower, upper) = DRS::read_drs_input_file(path);
        }
        utilization = DRS::call_drs(
            args.nodes,
            *u,
            lower,
            upper,
            all_periods,
            &node_distribution,
        );
    }
    utilization
}
