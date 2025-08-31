use rand::Rng;

pub struct UUnifast;

impl UUnifast {
    /// Calls uunifast algorithm and returns utilization vector with length n that adds up to u
    pub fn call_uunifast(n: i32, u: f64) -> Vec<f64> {
        println!(
            "\n\nDistributing total processor utilization U = {} with uunifast",
            u
        );
        let utilization: Vec<f64> = UUnifast::uunifast(n, u);
        assert!(utilization.len() == n as usize);
        utilization
    }

    /// UuniFast algorithm like described in IEEE paper https://ieeexplore.ieee.org/abstract/document/1311021
    /// Page 6, upper right before '4. Simulation results'
    /// To efficiently generate task sets with uniform distribution and with O(n) complexity
    fn uunifast(n: i32, u: f64) -> Vec<f64> {
        // Slide 7 on VL04_SchedulingPeriodic
        // U_i = C_i / T_i and sum of all U_i must be <= 1 (else not schedulable)
        assert!(
            u <= 1.0 && u >= 0.0,
            "The processor utilization should be value between 0 and 1"
        );
        let mut sum_u = u;
        let mut rng = rand::thread_rng();
        let mut vect_u: Vec<f64> = Vec::new();
        for i in 0..n - 1 {
            let next_sum_u = sum_u * rng.r#gen::<f64>().powf(1.0 / (n - i) as f64);
            vect_u.push(sum_u - next_sum_u);
            sum_u = next_sum_u;
        }
        vect_u.push(sum_u);
        println!("Utilization vector {:?}\n\n", vect_u);
        assert!(u - vect_u.iter().sum::<f64>().abs() < 1e-6); // floating point arithmetic
        vect_u
    }
}
