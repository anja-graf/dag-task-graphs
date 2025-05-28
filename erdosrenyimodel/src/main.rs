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

    fn new_erdos_renyi_var2(n: i32, m: i32) -> Self {
        let mut edges = Vec::new();
        let mut rng = rand::thread_rng();

        let mut counter = 0;
        while counter < m {
            let i = rng.gen_range(0..=n);
            let j = rng.gen_range(0..=n);
            println!("{:?}",(i,j));
            if i != j && ! edges.contains(&(i,j)) {
                edges.push((i, j));
                counter += 1;
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

    // Generate a comment like dot -Tpng graph.dot -o graph.png
    fn dot_to_svg(input_dot: &str, output_svg: &str) -> io::Result<()> {
        let status = Command::new("dot")
            .arg("-Tsvg")
            .arg(input_dot)
            .arg("-o")
            .arg(output_svg)
            .status()?;
    
        if status.success() {
            println!("SVG erfolgreich erstellt: {}", output_svg);
        } else {
            eprintln!("Fgraphehler beim Konvertieren mit Graphviz (dot).");
        }
    
        Ok(())
    }
}

fn main() -> Result<()> {
    let n = 3;
    

    let graph = Graph::new_erdos_renyi_var2(n, 2);

    graph.write_dot("graph.dot")?;
    Graph::dot_to_svg("graph.dot","graph.svg")?;

    println!("Graph gespeichert als 'graph.dot' ");
    Ok(())
}
