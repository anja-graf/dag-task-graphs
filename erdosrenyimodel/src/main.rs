use rand::Rng;
use std::fs::File;
use std::io::{Result, Write};
use std::process::Command;
use std::io;
use clap::Parser;
use std::collections::HashMap;
use std::collections::HashSet;

struct Graph {
    //nodes: i32,
    edges: Vec<(i32, i32)>,
}

impl Graph {
    fn new_erdos_renyi_var1(n: i32, p: f64) -> Self {
        assert!(0.0 <= p && p <= 1.0, "The probability has to be between 0 and 1!");
        let mut edges = Vec::new();
        let mut rng = rand::thread_rng();
        // In this adjacency list we store a list of reachable nodes from a key node
        let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();


        for i in 0..n {
            for j in 0..n {
                // Here we add each possible edge with probability p

                // Cycles would lead to non executable task arrangement
                // That is why we don't want to add edges that would close up a cycle
                // There should not be a path from j to i if we add the edge i to j
                if rng.r#gen::<f64>() < p && !Graph::has_path(&adj, j, i, &mut HashSet::new()) {
                    edges.push((i, j));
                    adj.entry(i).or_default().push(j);
                }
            }
        }

        // Graph { nodes: n, edges }
        Graph {edges }
    }

    fn new_erdos_renyi_var2(n: i32, m: i32) -> Self {
        // In directed graphes we can have twice the amount of edges in comparison to undirected graphes,
        // BUT as soon as we have more than an undirected complete graph there has to be a cycle
        assert!(m <= n*(n-1)/2,"The number of edges is higher than possible!"); //Kantenanzahl Vollständiger Graph, siehe: https://de.wikipedia.org/wiki/Vollst%C3%A4ndiger_Graph
        let mut edges = Vec::new();
        let mut rng = rand::thread_rng();
        // In this adjacency list we store a list of reachable nodes from a key node
        let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();

        let mut counter = 0;
        while counter < m {
            let i = rng.gen_range(0..n);
            let j = rng.gen_range(0..n);
            //println!("{:?}",edges);
            if i != j && !(edges.contains(&(i,j)) || edges.contains(&(j,i)))  {
                // Cycles would lead to non executable task arrangement
                // That is why we don't want to add edges that would close up a cycle
                // There should not be a path from j to i if we add the edge i to j
                if !Graph::has_path(&adj, j, i, &mut HashSet::new()) {
                    edges.push((i, j));
                    adj.entry(i).or_default().push(j);
                    // println!("{:?}",adj);
                    counter += 1;
                }
            } 
        }
        
        //Graph { nodes: n, edges }
        Graph {edges }
    }

    fn has_path (adj: &HashMap<i32, Vec<i32>>, source: i32, target: i32, visited: &mut HashSet<i32>) -> bool {
        if source == target { // Source is target so we have a (trivial) path
            return true;
        }
        if visited.contains(&source) { // We already visited our source which would lead to a cycle
            return false;
        }
        visited.insert(source); // Add current source node, so we won't check it again
        if let Some(neighbors) = adj.get(&source) {
            for &next in neighbors { // Check for each neighbor if we can reach target node
                if Graph::has_path(adj, next, target, visited) {
                    return true;
                }
            }
        }
        false // There is no path between source and target
    }

    fn write_dot(&self, filename: &str) -> Result<()> {
        let mut file = File::create(filename)?;
        //println!("Generate graph with {} nodes ",self.nodes);
        writeln!(file, "digraph {{")?;

        for (u, v) in &self.edges {
            writeln!(file, "    {} -> {};", u, v)?;
        }

        writeln!(file, "}}")?;
        println!("DOT successfully created '{}'", filename);
        Ok(())
    }

    // Generate a comment like dot -Tpng graph.dot -o graph.png
    fn dot_to_svg(input_dot: &str, output_svg: &str) -> io::Result<()> {
        let status = Command::new("dot")
            .arg("-Tsvg")
            .arg(input_dot)
            .arg("-o")
            .arg(output_svg)
            .status()?;
    
        if status.success() {
            println!("SVG successfully created '{}'", output_svg);
        } else {
            eprintln!("Error while converting with Graphviz (dot).");
        }
    
        Ok(())
    }
}

#[derive(Parser, Debug)]
#[command(name = "erdosrenyimodel", about = "Generates a random G(n, m) or G(n, p) graph")]
struct Args {
    /// Number of nodes
    #[arg(short, long)]
    nodes: i32,

    /// Number of edges
    #[arg(short, long, default_value="2")]
    edges: i32,

    /// Probability for edges
    #[arg(short, long, default_value="0.5")]
    probability: f64,

    /// path for output file in dot format
    #[arg(short, long, default_value = "graph.dot")]
    dot: String,

    /// path for output file in svg format
    #[arg(short, long, default_value = "graph.svg")]
    svg: String,

    /// option for variant 1 or 2
    #[arg(short, long, default_value = "1")]
    option: i32,
}

fn main() -> Result<()> {
    let args = Args::parse();
    assert!(args.nodes > 1,"There should be more than one node!");
    assert!(!(args.option == 1 && args.option == 2),"There is only option 1 or 2");
    assert!(args.dot != args.svg,"The names should not be both the same");


    if args.option == 1 {
        println!("The graph will be generated with option 1, nodes = {} and probability = {}", args.nodes, args.probability);
        let graph = Graph::new_erdos_renyi_var1(args.nodes, args.probability);
        graph.write_dot(&args.dot)?;
    } else if args.option == 2 {
        println!("The graph will be generated with option 2, nodes = {} and edges = {}", args.nodes, args.edges);
        let graph = Graph::new_erdos_renyi_var2(args.nodes,args.edges);
        graph.write_dot(&args.dot)?;
    }

    Graph::dot_to_svg(&args.dot,&args.svg)?;

    println!("Graph saved as '{}' and '{}'",&args.dot,&args.svg);
    Ok(())
}
