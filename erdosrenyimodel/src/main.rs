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
    assert!((args.dot != args.svg) && (args.dot != args.png) && (args.svg != args.png), "The path names should not be the same");

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
    let schedule = graph.distribute_parameters_uniprocessor();

    if let Some(u) = &args.uunifast {
        println!("Distributing total processor utilization U = {} with uunifast",u);
        graph.distribute_uunifast(*u);
    }
    // Save resulting graph in dot file, svg file and png file and save parameters of all tasks if wanted
    graph.write_dot(&args.dot, schedule)?;
    if let Some(path) = &args.csv {
        assert!((path != &args.dot) && (path != &args.svg) && (path != &args.png), "The path names should not be the same");
        graph.write_parameters(path)?; 
    }
    Graph::dot_to_svg_png(&args.dot, &args.svg, &args.png)?;
    println!("Graph saved as '{}', '{}' and '{}'", &args.dot, &args.svg, &args.png);
    Ok(())
}
