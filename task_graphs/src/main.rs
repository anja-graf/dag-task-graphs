mod interface;
mod graph;
mod output;
mod drs;
mod uunifast;
mod erdosrenyi;
mod schedule;

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
    let mut changed_comp:bool= false;

    // Distribute utilization by generating utilization vector and setting computation times new
    if let Some(u) = &args.uunifast_utilization {
        println!("Distributing total processor utilization U = {} with uunifast",u);
        graph.distribute_uunifast(*u);
        changed_comp = true;
    } else if let Some(u) = &args.drs_utilization {
        //TODO: read input file for drs
        println!("Distributing total processor utilization U = {} with drs",u);
        let mut lower = args.lower_bounds;
        let mut upper = args.upper_bounds;
        // If a drs input file is given, read it 
        if let Some(path) = &args.drs_path {
            (lower, upper) = Graph::read_drs_input_file(path);
        }
        graph.distribute_drs(args.nodes,*u,lower, upper);
        changed_comp = true;
    }

    // Save resulting graph in dot file, svg file and png file and save parameters of all tasks if wanted
    graph.write_dot(&args.dot, schedule,changed_comp)?;
    if let Some(path) = &args.csv {
        assert!((path != &args.dot) && (path != &args.svg) && (path != &args.png), "The path names should not be the same");
        graph.write_parameters(path)?; 
    }
    Graph::dot_to_svg_png(&args.dot, &args.svg, &args.png)?;
    println!("Graph saved as '{}', '{}' and '{}'", &args.dot, &args.svg, &args.png);
    Ok(())
}
