use crate::graph::Graph;
use rand::Rng;

impl Graph {
    /// Calls uunifast algorithm and assigns new computation times
    pub fn distribute_uunifast(&mut self, u: f64) {
        // Slide 7 on VL04_SchedulingPeriodic
        // U_i = C_i / T_i and sum of all U_i must be <= 1 (else not schedulable)
        let utilization: Vec<f64> = self.uunifast(u);
        assert!(utilization.len() == self.nodes.len());
        for i in 0..self.nodes.len() {
            self.nodes[i].computation_time =
                (self.nodes[i].abs_deadline as f64 * utilization[i]).round() as i32;

            if self.nodes[i].computation_time == 0 {
                println!(
                    "[WARNING] Computation time of task {} was set to 0!",
                    self.nodes[i].name
                );
            } else if self.nodes[i].computation_time
                > self.nodes[i].abs_deadline + self.nodes[i].start_time
            {
                println!(
                    "[ERROR] Computation time of task {} is too high to be executable before its deadline!",
                    self.nodes[i].name
                );
            }
        }
    }

    /// UuniFast algorithm like described in IEEE paper https://ieeexplore.ieee.org/abstract/document/1311021
    /// Page 6, upper right before '4. Simulation results'
    /// To efficiently generate task sets with uniform distribution and with O(n) complexity
    fn uunifast(&self, u: f64) -> Vec<f64> {
        assert!(
            u <= 1.0 && u >= 0.0,
            "The processor utilization should be percentage between 0 and 1"
        );
        let mut sum_u = u;
        let mut rng = rand::thread_rng();
        let mut vect_u: Vec<f64> = Vec::new();
        for i in 0..self.nodes.len() - 1 {
            let next_sum_u = sum_u * rng.r#gen::<f64>().powf(1.0 / (self.nodes.len() - i) as f64);
            vect_u.push(sum_u - next_sum_u);
            sum_u = next_sum_u;
        }
        vect_u.push(sum_u);
        println!("{:?}", vect_u);
        assert!(u - vect_u.iter().sum::<f64>().abs() < 1e-6); // floating point arithmetic
        vect_u
    }
}
