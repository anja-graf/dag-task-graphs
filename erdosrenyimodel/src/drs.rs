use crate::graph::Graph;
use std::process::Command;

impl Graph {
    pub fn distribute_drs(
        &mut self,
        n: i32,
        u: f64,
        lower_bounds: Option<Vec<f64>>,
        upper_bounds: Option<Vec<f64>>,
    ) {
        assert!(
            u <= 1.0 && u >= 0.0,
            "The processor utilization should be percentage between 0 and 1"
        );

        let upper: Vec<f64> = if let Some(ref upper) = upper_bounds {
            assert!(
                n == upper.len() as i32,
                "[ERROR] The length of upper bound sequence should be equal to the number of nodes"
            );
            assert!(
                u <= upper.iter().sum::<f64>().abs(),
                "[ERROR] The sum of the upper bound values should not be less than the required total utilization"
            );
            println!("Distributing upper bounds: {:?}", upper);
            upper.clone()
        } else {
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
            println!("Distributing lower bounds: {:?}", lower);
            lower.clone()
        } else {
            vec![0.0; n as usize]
        };

        println!(
            "{} nodes, total utilization: {}, lower bounds: {:?}, upper bounds: {:?}",
            n, u, lower, upper
        );
        let utilization: Vec<f64> = Self::call_python_drs(n, u, lower, upper);
        assert!(utilization.len() == self.nodes.len());
        for i in 0..self.nodes.len() {
            self.nodes[i].computation_time =
                (self.nodes[i].abs_deadline as f64 * utilization[i]).round() as i32;

            if self.nodes[i].computation_time == 0 {
                println!(
                    "[WARNING] Computation time of task {} was set to 0!",
                    self.nodes[i].name
                );
            } else if self.nodes[i].abs_deadline
                < self.nodes[i].computation_time + self.nodes[i].start_time
            {
                println!(
                    "[ERROR] Computation time of task {} is too high to be executable before its deadline!",
                    self.nodes[i].name
                );
            }
        }
    }

    fn call_python_drs(n: i32, u: f64, lower_bounds: Vec<f64>, upper_bounds: Vec<f64>) -> Vec<f64> {
        // call drs algorithm like described in IEEE paper https://ieeexplore.ieee.org/abstract/document/1311021
        let python_code = format!(
            "import json\nfrom drs import drs \nprint(json.dumps(drs({0}, {1},{2}, {3})))",
            n,
            u,
            format!("{:?}", upper_bounds),
            format!("{:?}", lower_bounds)
        );

        //println!("Python code to be executed: {}", python_code);

        let output = Command::new("python3")
            .arg("-c")
            .arg(&python_code)
            .output()
            .expect("failed to execute python");

        if output.status.success() {
            //println!("[SUCCESS] {:?}", output);
            let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
            //println!("[INFO] Python output: {}", stdout);
            let result_vec: Vec<f64> =
                serde_json::from_str(&stdout).expect("Failed to parse python output to vector");

            println!("Computed vector: {:?}", result_vec);
            return result_vec;
        } else {
            if String::from_utf8_lossy(&output.stderr).contains("ModuleNotFoundError") {
                println!(
                    "[ERROR] Could not distribute processor utilization, because the python module 'drs' is not installed"
                );
            } else {
                println!("[ERROR] {:?}", output);
            }
            return Vec::new();
        }
    }
}
