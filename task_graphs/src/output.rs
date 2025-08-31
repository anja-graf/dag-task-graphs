use crate::graph::Graph;
use std::fs::File;
use std::io;
use std::io::{Result, Write};
use std::process::Command;

impl Graph {
    /// Writes the current graph with all its edges and nodes and their parameters into dot file.
    /// See also <https://graphviz.org/doc/info/lang.html>
    pub fn write_dot(&self, path: &str, schedule: Vec<i32>, uunifast: bool) -> Result<()> {
        let mut file = File::create(path)?;
        writeln!(file, "digraph {{")?;

        let mut period = -1;
        if let Some(first_node) = self.nodes.first() {
            // all graphs have at least one node
            period = first_node.period; // T_1 = T_2 = ... = T_n
        }

        // If uunifast is used the computation times can be outside of {10..100}
        let mut ctext = "";
        if !uunifast {
            ctext = "is set to random value in {10..100}";
        }

        // Description of labels
        writeln!(
            file,
            "label = <<br/>
                        <b>Task Parameter</b> <br/>
                        Computation time C<sub>i</sub> {} <br/>
                        Arrival/Release time a<sub>i</sub> is set to 0 <br/>
                        Absolute Deadline d<sub>i</sub> is set to T<sub>i</sub> <br/>
                        Relative Deadline D<sub>i</sub> = d<sub>i</sub> - a<sub>i</sub> <br/>
                        Start time s<sub>i</sub> <br/>
                        Finishing time f<sub>i</sub> = s<sub>i</sub> + C<sub>i</sub> <br/>
                        Response time R<sub>i</sub> = f<sub>i</sub> - a<sub>i</sub><br/> 
                        Lateness L<sub>i</sub> = f<sub>i</sub> - d<sub>i</sub> <br/>
                        Period T<sub>i</sub> is set to sum of all C<sub>i</sub> = {} <br/>
                        Priority of task is defined by its number <br/>
                        Schedule: {:?}
                        >",
            ctext, period, schedule
        )?;

        // Explicit definiton of all nodes for labeling each
        for node in &self.nodes {
            writeln!(
                file,
                "\t{} [label=\"{0}\",xlabel=<s<sub>{0}</sub> = {} <br/> C<sub>{0}</sub> = {} <br/> f<sub>{0}</sub> = {}>];",
                node.name, node.start_time, node.computation_time, node.finishing_time
            )?;
        }

        // List all edges of graph
        for (u, v) in &self.edges {
            writeln!(file, "\t{} -> {};", u, v)?;
        }

        writeln!(file, "}}")?;
        //println!("DOT successfully created '{}'", path);
        Ok(())
    }

    /// Generates an output file including all parameters of the nodes in the graph
    pub fn write_parameters(&self, path: &str) -> Result<()> {
        let mut file = File::create(path)?;
        // Column description
        writeln!(
            file,
            "Task/Priority\tStart time\tComputation time\tAbsolute deadline\tRelative deadline\tFinishing time\tResponse time\tLateness\tArrival time\tPeriod\tUtilization"
        )?;
        // Row for each node
        for node in &self.nodes {
            writeln!(
                file,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                node.name,
                node.start_time,
                node.computation_time,
                node.abs_deadline,
                node.rel_deadline,
                node.finishing_time,
                node.response_time,
                node.lateness,
                node.arrival_time,
                node.period,
                node.computation_time as f64 / node.abs_deadline as f64
            )?;
        }
        println!("Parameter file successfully created '{}'", path);
        Ok(())
    }

    /// Generates a comment to convert a dot file into svg and png with graphviz
    /// e.g. dot -Tsvg graph.dot -o graph.svg
    /// or dot -Tpng graph.dot -o graph.png
    pub fn dot_to_svg_png(
        input_dot_path: &str,
        output_svg_path: &str,
        output_png_path: &str,
    ) -> io::Result<()> {
        // Check if graphviz is installed
        let mut is_installed = false;
        if let Ok(output) = Command::new("which").arg("dot").output() {
            is_installed = output.status.success();
        }
        if !is_installed {
            panic!(
                "Could not generate svg file, because graphviz does not seem to be installed! See the ReadMe or https://graphviz.org/ for installation instructions"
            );
        }
        // Generate svg
        let status = Command::new("dot")
            .arg("-Tsvg")
            .arg(input_dot_path)
            .arg("-o")
            .arg(output_svg_path)
            .status()?;

        if !status.success() {
            panic!("Error while converting dot to svg");
        }

        // Generate png
        let status = Command::new("dot")
            .arg("-Tpng")
            .arg(input_dot_path)
            .arg("-o")
            .arg(output_png_path)
            .status()?;

        if !status.success() {
            panic!("Error while converting dot to png");
        }

        Ok(())
    }
}
