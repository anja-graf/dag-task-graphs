use crate::graph::Graph;
use std::fs::File;
use std::io;
use std::io::{ Result, Write };
use std::process::Command;

impl Graph {
    /// Writes the current graph with all its edges and nodes and their parameters into dot file.
    /// See also <https://graphviz.org/doc/info/lang.html>
    pub fn write_dot(&self, path: &str) -> Result<()> {
        let mut file = File::create(path)?;
        writeln!(file, "digraph {{")?;

        // Description of labels
        writeln!(
            file,
            "label = <<br/>
                         Arrival time a<sub>i</sub> <br/>
                         Start time s<sub>i</sub><br/>
                         Absolute Deadline d<sub>i</sub>>"
        )?;

        // Explicit definiton of all nodes for labeling each
        for node in &self.nodes {
            writeln!(
                file,
                "\t{} [label=\"{0}\",xlabel=<a<sub>{0}</sub> = {} <br/> s<sub>{0}</sub> = {} <br/> d<sub>{0}</sub> = {}>];",
                node.name,
                node.arrival_time,
                node.start_time,
                node.deadline
            )?;
        }

        // List all edges of graph
        for (u, v) in &self.edges {
            writeln!(file, "\t{} -> {};", u, v)?;
        }

        writeln!(file, "}}")?;
        println!("DOT successfully created '{}'", path);
        Ok(())
    }

    /// Generates a comment to convert a dot file into svg with graphviz
    /// e.g. dot -Tsvg graph.dot -o graph.svg
    pub fn dot_to_svg(input_dot_path: &str, output_svg_path: &str) -> io::Result<()> {
        let status = Command::new("dot")
            .arg("-Tsvg")
            .arg(input_dot_path)
            .arg("-o")
            .arg(output_svg_path)
            .status()?;

        if status.success() {
            println!("SVG successfully created '{}'", output_svg_path);
        } else {
            panic!("Error while converting dot to svg");
        }

        Ok(())
    }
}
