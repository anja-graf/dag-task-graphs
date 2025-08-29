mod interface;
mod graph;
mod output;
mod drs;
mod uunifast;
mod erdosrenyi;
mod schedule;

use graph::Graph;
use uunifast::UUnifast;
use drs::DRS;

use interface::Args;
use clap::Parser;
use clap::CommandFactory;
use std::io::Result;
use rand::Rng;

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
    assert!(!(args.option == 1 && args.option == 2), "There is only option 1 or 2");

    let node_distribution:Vec<i32> = generate_random_node_distribution(args.graphs, args.nodes);
    let mut edge_distribution:Vec<i32> = Vec::new();
    if args.option == 2 {
        edge_distribution = generate_random_edge_distribution(&node_distribution,args.edges);
    }
    
    let mut all_graphs:Vec<Graph> = Vec::new();
    let mut all_periods: Vec<i32> = Vec::new();

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
            graph = Graph::new_erdos_renyi_var2(node_distribution[i as usize], edge_distribution[i as usize]);
        } else {
            panic!("Error: Invalid option!");
        }

        if let Some(first_node) = graph.nodes.first() { // all graphs have at least one node
            all_periods.push(first_node.period); // T_1 = T_2 = ... = T_n
        }

        all_graphs.push(graph);
    }



    // Distribute utilization by generating utilization vector and setting computation times new
    let mut utilization:Vec<f64> = Vec::new();
    

    if let Some(u) = &args.uunifast_utilization {
        utilization = UUnifast::call_uunifast(args.nodes,*u);
        
    } else if let Some(u) = &args.drs_utilization {
        let mut lower = args.lower_bounds;
        let mut upper = args.upper_bounds;
        // If a drs input file is given, read it 
        if let Some(path) = &args.drs_path {
            (lower, upper) = DRS::read_drs_input_file(path);
        }
        utilization = DRS::call_drs(args.nodes, *u, lower, upper, all_periods,&node_distribution);
    }

    let mut offset = 0;
    for (i,graph) in all_graphs.iter_mut().enumerate() {
            // Save resulting graph in dot file, svg file and png file and save parameters of all tasks if wanted
        // Generating task parameters like start time or deadline
        

        //Distribute utilization
        let mut sub_u:Vec<f64> = Vec::new();
        if args.uunifast_utilization.is_some() || args.drs_utilization.is_some(){
            //sub_u = utilization[..graph.nodes.len().min(utilization.len())].to_vec();

            sub_u = utilization[offset..offset + node_distribution[i] as usize].to_vec();
            offset += node_distribution[i] as usize;

            println!("\n\nSubvector for utilization of graph {}: {:?}",i, sub_u);
            //graph.distribute_parameters(&sub_u);
        } 

        let schedule = graph.distribute_parameters_uniprocessor(&sub_u);

        let dot = format!("{}{}.dot",args.dot, i + 1);
        graph.write_dot(&dot, schedule, args.uunifast_utilization.is_some())?;
        if let Some(csv) = &args.csv {
            graph.write_parameters(&format!("{}{}.csv",csv, i+1))?; 
        }
        let svg = &format!("{}{}.svg",args.svg,i+1);
        let png = &format!("{}{}.png",args.png,i+1);
        Graph::dot_to_svg_png(&dot, &svg, &png)?;
        println!("Graph {} saved as '{}', '{}' and '{}'",i + 1, &dot, &svg, &png);

    }


    Ok(())
}


fn generate_random_node_distribution(graphs:i32,nodes:i32) -> Vec<i32> {
    assert!(nodes >= graphs, "There should be at least one node for each graph");
    let mut distribution = vec![1; graphs as usize];
    let mut rng = rand::thread_rng();
    let rest = nodes - graphs;
    for _ in 0..rest {
        distribution[rng.gen_range(0..graphs) as usize] += 1;
    }
    println!("Nodes were distributed on graphs: {:?}", distribution);
    distribution
}

fn generate_random_edge_distribution(node_distribution: &Vec<i32>, total_edges: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut remaining = total_edges;
    let mut distribution = vec![0; node_distribution.len()];

    // calculate max edges for each graph
    let max_edges: Vec<i32> = node_distribution
        .iter()
        .map(|&n| n * (n - 1) / 2)
        .collect();

    assert!(
        total_edges <= max_edges.iter().sum(),
        "Edges can't be distributed among the graphs, because number of edges is too high!");

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