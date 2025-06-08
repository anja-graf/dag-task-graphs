mod interface;
mod graph;
mod output;

use graph::Graph;
use interface::Args;
use clap::Parser;
use clap::CommandFactory;
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
    assert!(!(args.option == 1 && args.option == 2), "There is only option 1 or 2");
    assert!(args.dot != args.svg, "The path names should not be both the same");

    // Depending on the specified option a function is used to generate the graph
    let mut graph;
    if args.option == 1 {
        println!(
            "The graph will be generated with option 1, nodes = {} and probability = {}",
            args.nodes,
            args.probability
        );
        graph = Graph::new_erdos_renyi_var1(args.nodes, args.probability);
    } else if args.option == 2 {
        println!(
            "The graph will be generated with option 2, nodes = {} and edges = {}",
            args.nodes,
            args.edges
        );
        graph = Graph::new_erdos_renyi_var2(args.nodes, args.edges);
    } else {
        panic!("Error: Invalid option!");
    }

    // Generating task parameters like start time or deadline
    graph.distribute_parameters();

    // Save resulting graph in dot file and svg file
    graph.write_dot(&args.dot)?;
    Graph::dot_to_svg(&args.dot, &args.svg)?;
    println!("Graph saved as '{}' and '{}'", &args.dot, &args.svg);
    Ok(())
}
