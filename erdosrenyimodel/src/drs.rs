use crate::graph::Graph;
use std::process::Command;

use std::fs;
use std::collections::HashMap;

impl Graph {
    /// Distributes total processor utilization among the nodes of the graph
    /// by assigning new computation times, optionally constrained by lower and upper bounds
    pub fn distribute_drs(
        &mut self,
        n: i32,
        u: f64,
        lower_bounds: Option<Vec<f64>>,
        upper_bounds: Option<Vec<f64>>,
    ) {
        assert!(
            u <= 1.0 && u >= 0.0,
            "The processor utilization should be value between 0 and 1"
        );

        // Assert minimal requirements for useful upper and lower bounds if they are given
        // else set default values that help to generate feasible utilization
        let upper: Vec<f64> = if let Some(ref upper) = upper_bounds {
            assert!(
                n == upper.len() as i32,
                "[ERROR] The length of upper bound sequence should be equal to the number of nodes"
            );
            assert!(
                u <= upper.iter().sum::<f64>().abs(),
                "[ERROR] The sum of the upper bound values should not be less than the required total utilization"
            );
            upper.clone()
        } else {
            println!("Generating default upper bounds that ensure the task can be executed before its deadline");
            self.nodes
                .iter()
                .map(|node| node.computation_time as f64 / node.rel_deadline as f64)
                .collect()
        };
        let lower: Vec<f64> = if let Some(ref lower) = lower_bounds {
            assert!(
                n == lower.len() as i32,
                "[ERROR] The length of lower bound sequence should be equal to the number of nodes"
            );
            assert!(
                u >= lower.iter().sum::<f64>().abs(),
                "[ERROR] The sum of the lower bound values should not be greater than the required total utilization"
            );
            lower.clone()
        } else {
            println!("Generating default lower bounds that ensure the task has an execution time of at least 10");
            // vec![0.0; n as usize]
            // ensure that computation time is >= 10
            self.nodes
                .iter()
                .map(|node| 10.0 / node.rel_deadline as f64)
                .collect()
        };

        println!(
            "Lower bounds: {:?} \nUpper bounds: {:?}",
            lower, upper
        );

        for i in 0..n {
            assert!(lower[i as usize] <= upper[i as usize],"[ERROR] The lower bound for each node should not be greater than the upper bound!");
        }


        // Call python script to compute utilization distribution
        let utilization: Vec<f64> = Self::call_python_drs(n, u, lower, upper);
        assert!(utilization.len() == self.nodes.len());

        // Distribute the computed utilization to the nodes
        self.distribute_parameters(&utilization);
    }

    /// Internal helper that invokes a python script to compute the utilization distribution 
    /// using the DRS algorithm
    fn call_python_drs(n: i32, u: f64, lower_bounds: Vec<f64>, upper_bounds: Vec<f64>) -> Vec<f64> {
        // call drs algorithm https://pypi.org/project/drs/
        // informative IEEE paper https://ieeexplore.ieee.org/abstract/document/1311021
        let python_code = format!(
            "import json\nfrom drs import drs \nprint(json.dumps(drs({0}, {1},{2}, {3})))",
            n,
            u,
            format!("{:?}", upper_bounds),
            format!("{:?}", lower_bounds)
        );

        //println!("Python code to be executed: {}", python_code);

        // starting new python process
        let output = Command::new("python3")
            .arg("-c")
            .arg(&python_code)
            .output()
            .expect("failed to execute python");

        // Evaluate the output and extract the result
        if output.status.success() {
            let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
            let result_vec: Vec<f64> =
                serde_json::from_str(&stdout).expect("Failed to parse python output to vector");
            println!("Computed vector: {:?}", result_vec);
            return result_vec;
        } else {
            if String::from_utf8_lossy(&output.stderr).contains("ModuleNotFoundError") {
                panic!(
                    "[ERROR] Could not distribute processor utilization, because the python module 'drs' is not installed"
                );
            } else {
                //panic!("[ERROR] {}", String::from_utf8_lossy(&output.stderr));
                panic!("[ERROR] Utilization calculation could not be performed, please try another total utilization!")
            }
        }
    }

    pub fn read_drs_input_file(path: &str) -> (Option<Vec<f64>>, Option<Vec<f64>>) {
        println!("Reading drs input file from '{}'", path);

            let content = match fs::read_to_string(path) {
                Ok(input) => input,
                Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
                panic!("[ERROR] File '{}' not found for drs input", path);
                }
                Err(e) => {
                panic!("[ERROR] Could not read from filepath '{}' for drs input: {}", path, e);
                }
            };
            let data: HashMap<String, Vec<f64>> = toml::from_str(&content).unwrap();

            
            let lower = match data.get("min") {
                Some(input) => Some(input.clone()),
                None => {
                    println!("Could not read lower bounds: 'min = []' is missing");
                    None
                }
            };
            let upper = match data.get("max") {
                Some(input) => Some(input.clone()),
                None => {
                    println!("Could not read upper bounds: 'max = []' is missing");
                    None
                }
            };
            return (lower, upper);
    }
}
