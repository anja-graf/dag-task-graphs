use rand::Rng;
use std::fs::File;
use std::io::{Result, Write};
use std::process::Command;
use std::io;

struct Graph {
    nodes: i32,
    edges: Vec<(i32, i32)>,
}

impl Graph {
    fn new_erdos_renyi_var1(n: i32, p: f64) -> Self {
        let mut edges = Vec::new();
        let mut rng = rand::thread_rng();

        for i in 0..n {
            for j in (i + 1)..n {
                if rng.r#gen::<f64>() < p {
                    edges.push((i, j));
                }
            }
        }

        Graph { nodes: n, edges }
    }


    fn write_dot(&self, filename: &str) -> Result<()> {
        let mut file = File::create(filename)?;
        println!("Generate graph with {} nodes ",self.nodes);
        writeln!(file, "graph {{")?;

        for (u, v) in &self.edges {
            writeln!(file, "    {} -- {};", u, v)?;
        }

        writeln!(file, "}}")?;
        Ok(())
    }

}

fn main() -> Result<()> {
    let n = 3;
    let p = 0.3;

    let graph = Graph::new_erdos_renyi_var1(n, p);

    graph.write_dot("graph.dot")?;

    println!("Graph gespeichert als 'graph.dot' ");
    Ok(())
}
